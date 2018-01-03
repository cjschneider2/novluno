defmodule RmServer.Application do
  # See https://hexdocs.pm/elixir/Application.html
  # for more information on OTP Applications
  @moduledoc false

  use Application

  def start(_type, _args) do
    # List all child processes to be supervised
    children = [
      # Starts a worker by calling: RmServer.Worker.start_link(arg)
      # {RmServer.Worker, arg},
      {Task.Supervisor, name: RmServer.TaskSupervisor},
      Supervisor.child_spec({Task, fn -> RmServer.accept() end},
                            restart: :permanent)
    ]

    # See https://hexdocs.pm/elixir/Supervisor.html
    # for other strategies and supported options
    opts = [strategy: :one_for_one, name: RmServer.Supervisor]
    Supervisor.start_link(children, opts)
  end
end
