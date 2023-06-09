job "metagame" {
  type = "service"

  update {
    max_parallel = 1
    stagger      = "10s"
  }

  group "api" {
    count = 1

    network {
      port "http" {
        static = 8067
      }
    }

    task "api" {
      driver = "docker"

      config {
        image = "ghcr.io/genudine/metagame/metagame:latest"
        ports = ["http"]
      }
    }
  }
}