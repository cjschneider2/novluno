require Logger

defmodule RmServer do
  @moduledoc """
  Documentation for RmServer.
  """

  def accept() do
    port = 10101
    addr = {192,168,56,1}
    Logger.info(fn -> "Using port:#{port}, addr:#{inspect(addr)}" end)
    {:ok, socket} =
      :gen_tcp.listen(port,
        [:list,
          {:ip, addr},
          packet: :raw,
          active: false,
          reuseaddr: true])
    Logger.info(fn -> "Accepting connections on #{inspect(socket)}" end)
    loop_acceptor(socket)
  end

  defp loop_acceptor(socket) do
    {:ok, client} = :gen_tcp.accept(socket)
    Logger.info(fn -> "Connected client #{inspect(client)}" end)
    {:ok, pid} = Task.Supervisor.start_child(RmServer.TaskSupervisor, fn -> serve(client) end)
    :ok = :gen_tcp.controlling_process(client, pid)
    loop_acceptor(socket)
  end

  defp serve(socket) do
    msg =
      case read_msg(socket) do
        {:ok, data} ->
          Logger.info("Got encrypted packet #{inspect(data)}")
          {:ok, data} = RmServer.Crypto.decode(data)
          Logger.info("decrypted packet #{inspect(data)}")
        {:error, :closed} ->
          Logger.info("Connection closed by client: exiting")
          exit(:shutdown)
        {:error, error} ->
          Logger.info("Unknown error: exiting")
          exit(error)
      end
    Logger.info(msg)
    serve(socket)
  end

  defp read_msg(socket) do
    :gen_tcp.recv(socket, 0)
  end

  defp send_msg(line, socket) do
    :gen_tcp.send(socket, line)
  end

end
