terraform {
  required_providers {
    docker = {
      source  = "kreuzwerker/docker"
      version = "3.0.2"
    }
  }
}

provider "docker" {
}

resource "docker_image" "postgres" {
  name         = "docker.io/postgres:15"
  keep_locally = true
}

locals {
  postgres_port     = 5432
  postgres_user     = "piston"
  postgres_password = "password"
  postgres_db       = "piston"
}

resource "docker_container" "postgres" {
  name  = "se-piston-dev-postgres"
  image = docker_image.postgres.image_id

  env = [
    "POSTGRES_USER=${ local.postgres_user }",
    "POSTGRES_PASSWORD=${ local.postgres_password }",
    "POSTGRES_DB=${ local.postgres_db }",
  ]

  ports {
    internal = 5432
    external = local.postgres_port
  }
}
