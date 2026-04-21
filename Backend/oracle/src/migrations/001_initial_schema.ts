import { Migration } from "./Migration";
import { OracleDatabase } from "../database/connection";
import { UserModel } from "../models/User";
import { TransactionModel } from "../models/Transaction";
import { Logger } from "../utils/logger";

export class InitialSchema extends Migration {
  private logger = Logger.getInstance();

  async up(): Promise<void> {
    const db = OracleDatabase.getInstance();

    try {
      this.logger.info("Running migration: InitialSchema (up)");

      // Create users table
      await db.execute(UserModel.getSequenceSQL());
      await db.execute(UserModel.getCreateTableSQL());
      this.logger.info("Created users table");

      // Create transactions table
      await db.execute(TransactionModel.getSequenceSQL());
      await db.execute(TransactionModel.getCreateTableSQL());
      this.logger.info("Created transactions table");
    } catch (error) {
      this.logger.error("Migration up failed:", error);
      throw error;
    }
  }

  async down(): Promise<void> {
    const db = OracleDatabase.getInstance();

    try {
      this.logger.info("Running migration: InitialSchema (down)");

      await db.execute(`DROP TABLE ${TransactionModel.tableName}`);
      await db.execute(`DROP SEQUENCE ${TransactionModel.tableName}_seq`);

      await db.execute(`DROP TABLE ${UserModel.tableName}`);
      await db.execute(`DROP SEQUENCE ${UserModel.tableName}_seq`);

      this.logger.info("Dropped tables");
    } catch (error) {
      this.logger.error("Migration down failed:", error);
      throw error;
    }
  }
}
