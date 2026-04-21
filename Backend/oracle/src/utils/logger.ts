import * as fs from "fs";
import * as path from "path";

type LogLevel = "error" | "warn" | "info" | "debug";

export class Logger {
  private static instance: Logger;
  private logLevel: LogLevel;
  private logsDir: string;

  private constructor() {
    this.logLevel = (process.env.LOG_LEVEL as LogLevel) || "info";
    this.logsDir = path.join(process.cwd(), "logs");
    this.ensureLogsDirectory();
  }

  static getInstance(): Logger {
    if (!Logger.instance) {
      Logger.instance = new Logger();
    }
    return Logger.instance;
  }

  private ensureLogsDirectory(): void {
    if (!fs.existsSync(this.logsDir)) {
      fs.mkdirSync(this.logsDir, { recursive: true });
    }
  }

  private formatMessage(level: LogLevel, message: string, data?: any): string {
    const timestamp = new Date().toISOString();
    const dataStr = data ? ` ${JSON.stringify(data)}` : "";
    return `[${timestamp}] [${level.toUpperCase()}] ${message}${dataStr}`;
  }

  private log(level: LogLevel, message: string, data?: any): void {
    const formatted = this.formatMessage(level, message, data);
    console.log(formatted);

    // Write to file
    const logFile = path.join(this.logsDir, `${level}.log`);
    fs.appendFileSync(logFile, formatted + "\n");
  }

  error(message: string, error?: any): void {
    this.log("error", message, error instanceof Error ? error.message : error);
  }

  warn(message: string, data?: any): void {
    this.log("warn", message, data);
  }

  info(message: string, data?: any): void {
    this.log("info", message, data);
  }

  debug(message: string, data?: any): void {
    if (this.logLevel === "debug") {
      this.log("debug", message, data);
    }
  }
}
