import dotenv from "dotenv";
import { OracleDatabase } from "./database/connection";
import { Logger } from "./utils/logger";

dotenv.config();

const logger = Logger.getInstance();

async function initializeApplication(): Promise<void> {
  try {
    logger.info("Initializing Oracle Service...");

    const database = OracleDatabase.getInstance();
    await database.initialize();

    logger.info("Oracle Service initialized successfully");

    // Example: Get connection info
    const poolInfo = await database.getPoolInfo();
    logger.info("Database pool info:", poolInfo);
  } catch (error) {
    logger.error("Failed to initialize application:", error);
    process.exit(1);
  }
}

export { OracleDatabase };

if (require.main === module) {
  initializeApplication().catch((error) => {
    logger.error("Unhandled error:", error);
    process.exit(1);
  });
}
