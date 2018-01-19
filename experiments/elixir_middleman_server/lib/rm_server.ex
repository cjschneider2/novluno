require Logger

defmodule RmServer do
  @moduledoc """
  Documentation for RmServer.
  """

  @port 10101
  @client_addr {192,168,56,1}
  @server_addr {198,24,149,46} # NOTE: using `rm.redmoonclassic.com` *sorry*

  def accept() do
    Logger.info(fn -> "Using port:#{@port}, addr:#{inspect(@client_addr)}" end)
    {:ok, socket} =
      :gen_tcp.listen(@port,
        [:list,
          {:ip, @client_addr},
          packet: :raw,
          active: false,
          reuseaddr: true])
    Logger.info(fn -> "Accepting connections on #{inspect(socket)}" end)
    loop_acceptor(socket)
  end

  defp loop_acceptor(socket) do
    # accept the client connection
    {:ok, client} = :gen_tcp.accept(socket)
    Logger.info(fn -> "Connected client #{inspect(client)}" end)

    # make a connection with the actual rm server
    {:ok, server} = :gen_tcp.connect(@server_addr, @port, [:list, active: :false])
    Logger.info(fn -> "Connected to RM server #{inspect(server)}" end)

    # start the monitoring server task
    {:ok, pid} =
      Task.Supervisor.start_child(
        RmServer.TaskSupervisor,
        fn -> serve(client, server) end
      )

    # we need to transfer control of the connection to the client process so if
    # the client process dies then the connection will go with it
    :ok = :gen_tcp.controlling_process(client, pid)
    :ok = :gen_tcp.controlling_process(server, pid)

    # do it all over again
    loop_acceptor(socket)
  end

  defp serve(client_socket, server_socket) do
    # receive messages from client
    client_msg =
      case read_msg(client_socket) do
        {:ok, data} ->
          Logger.info("Got encrypted packet #{inspect(data)}")
          {:ok, decoded_data} = RmServer.Crypto.decode(data)
          Logger.info("decrypted packet #{inspect(decoded_data)}")
          data
        {:error, :closed} ->
          Logger.info("Client connection closed by client: exiting")
          exit(:shutdown)
        {:error, error} ->
          Logger.error("Unknown error with client connection: exiting")
          exit(error)
      end

    # send client msg to the server
    result = send_msg(client_msg, server_socket)
    Logger.info("Sent packet to server -> result: #{inspect(result)}")

    # recv messages from server
    srv_msg = case read_msg(server_socket) do
      {:ok, data} ->
        Logger.info("Got encrypted packet #{inspect(data)}")
        {:ok, decoded_data} = RmServer.Crypto.decode(data)
        Logger.info("decrypted packet #{inspect(decoded_data)}")
        data
      {:error, :closed} ->
        Logger.info("server connection closed by server: exiting")
        exit(:shutdown)
      {:error, error} ->
        Logger.error("Unknown error `#{inspect(error)}` with server connection: exiting")
        exit(error)
    end

    # send server messages to the client
    response = send_msg(srv_msg, client_socket)
    Logger.info("Sent packet to client -> result: #{inspect(result)}")

    # loop
    serve(client_socket, server_socket)
  end

  defp read_msg(socket) do
    :gen_tcp.recv(socket, 0)
  end

  defp send_msg(line, socket) do
    :gen_tcp.send(socket, line)
  end

end
