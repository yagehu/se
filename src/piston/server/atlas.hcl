env "development" {
  src = "file://src/piston/server/schema.hcl"
  url = "postgres://piston:password@localhost:5432/piston?sslmode=disable"
  dev = "docker://postgres/15/dev?search_path=public"

  migration {
    dir    = "file://src/piston/server/migrations"
    format = atlas
  }
}
