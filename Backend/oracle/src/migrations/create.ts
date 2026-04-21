import * as fs from "fs";
import * as path from "path";

const migrationsDir = __dirname;

function createMigration(name: string): void {
  const timestamp = Date.now();
  const fileName = `${timestamp}_${name}.ts`;
  const filePath = path.join(migrationsDir, fileName);

  const template = `import { Migration } from './Migration';
import { OracleDatabase } from '../database/connection';
import { Logger } from '../utils/logger';

export class ${toCamelCase(name)} extends Migration {
  private logger = Logger.getInstance();

  async up(): Promise<void> {
    const db = OracleDatabase.getInstance();
    
    try {
      this.logger.info('Running migration: ${toCamelCase(name)} (up)');
      // TODO: Add your migration logic here
    } catch (error) {
      this.logger.error('Migration up failed:', error);
      throw error;
    }
  }

  async down(): Promise<void> {
    const db = OracleDatabase.getInstance();
    
    try {
      this.logger.info('Running migration: ${toCamelCase(name)} (down)');
      // TODO: Add your rollback logic here
    } catch (error) {
      this.logger.error('Migration down failed:', error);
      throw error;
    }
  }
}
`;

  fs.writeFileSync(filePath, template);
  console.log(`Migration created: ${fileName}`);
}

function toCamelCase(str: string): string {
  return str
    .replace(/[-_\s]+(.)?/g, (_, char) => (char ? char.toUpperCase() : ""))
    .replace(/^./, (char) => char.toUpperCase());
}

const migrationName = process.argv[2];
if (!migrationName) {
  console.error("Usage: ts-node create.ts <migration-name>");
  process.exit(1);
}

createMigration(migrationName);
