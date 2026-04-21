import oracledb from "oracledb";
import { Logger } from "../utils/logger";

export interface OracleConnectionConfig {
  user: string;
  password: string;
  connectionString: string;
  poolMin: number;
  poolMax: number;
  poolIncrement: number;
}

export class OracleDatabase {
  private static instance: OracleDatabase;
  private pool: oracledb.Pool | null = null;
  private logger: Logger;

  private constructor() {
    this.logger = Logger.getInstance();
  }

  static getInstance(): OracleDatabase {
    if (!OracleDatabase.instance) {
      OracleDatabase.instance = new OracleDatabase();
    }
    return OracleDatabase.instance;
  }

  async initialize(): Promise<void> {
    try {
      const config: OracleConnectionConfig = {
        user: process.env.ORACLE_USER || "",
        password: process.env.ORACLE_PASSWORD || "",
        connectionString: process.env.ORACLE_CONNECTION_STRING || "",
        poolMin: parseInt(process.env.ORACLE_POOL_MIN || "2", 10),
        poolMax: parseInt(process.env.ORACLE_POOL_MAX || "10", 10),
        poolIncrement: parseInt(process.env.ORACLE_POOL_INCREMENT || "1", 10),
      };

      if (!config.user || !config.password || !config.connectionString) {
        throw new Error("Missing required Oracle database configuration");
      }

      this.pool = await oracledb.createPool({
        user: config.user,
        password: config.password,
        connectString: config.connectionString,
        poolMin: config.poolMin,
        poolMax: config.poolMax,
        poolIncrement: config.poolIncrement,
      });

      this.logger.info("Oracle connection pool created successfully");
    } catch (error) {
      this.logger.error("Failed to initialize Oracle connection pool:", error);
      throw error;
    }
  }

  async getConnection(): Promise<oracledb.Connection> {
    if (!this.pool) {
      throw new Error("Database pool not initialized");
    }
    return await this.pool.getConnection();
  }

  async execute<T = any>(
    sql: string,
    binds: any[] = [],
    options: oracledb.ExecuteOptions = {},
  ): Promise<oracledb.Result<T>> {
    const connection = await this.getConnection();
    try {
      return await connection.execute<T>(sql, binds, options);
    } finally {
      await connection.close();
    }
  }

  async query<T = any>(sql: string, binds: any[] = []): Promise<T[]> {
    const result = await this.execute<T>(sql, binds, {
      outFormat: oracledb.OUT_FORMAT_OBJECT,
    });
    return (result.rows || []) as T[];
  }

  async getPoolInfo(): Promise<object> {
    if (!this.pool) {
      throw new Error("Database pool not initialized");
    }
    return this.pool.getStatistics();
  }

  async closePool(): Promise<void> {
    if (this.pool) {
      await this.pool.close();
      this.pool = null;
      this.logger.info("Oracle connection pool closed");
    }
  }
}
