export abstract class Migration {
  abstract up(): Promise<void>;
  abstract down(): Promise<void>;

  getName(): string {
    return this.constructor.name;
  }
}
