import dotenv from "dotenv";
import { OracleDatabase } from "../database/connection";
import { InitialSchema } from "./001_initial_schema";
import { Logger } from "../utils/logger";

dotenv.config();

const logger = Logger.getInstance();

async function runMigrations(): Promise<void> {
  try {
    logger.info("Starting database migrations...");

    const db = OracleDatabase.getInstance();
    await db.initialize();

    const migrations = [new InitialSchema()];

    for (const migration of migrations) {
      logger.info(`Running migration: ${migration.getName()}`);
      await migration.up();
      logger.info(`Completed migration: ${migration.getName()}`);
    }

    await db.closePool();
    logger.info("All migrations completed successfully");
  } catch (error) {
    logger.error("Migration failed:", error);
    process.exit(1);
  }
}

if (require.main === module) {
  runMigrations();
}
