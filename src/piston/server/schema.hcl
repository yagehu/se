schema "public" {
}

table "namespace" {
  schema = schema.public

  column "id" {
    null = false
    type = uuid
  }

  column "created_at" {
    null = false
    type = timestamptz
  }

  column "name" {
    null = false
    type = varchar(1024)
  }
}
