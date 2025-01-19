CREATE TABLE IF NOT EXISTS "test_table" (
  "id" UUID NOT NULL DEFAULT gen_random_uuid(),
  "name" TEXT NOT NULL,
  
  PRIMARY KEY ("id")
);

INSERT INTO "test_table" ("name") VALUES ('Uha');