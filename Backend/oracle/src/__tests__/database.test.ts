import { OracleDatabase } from "../database/connection";

describe("OracleDatabase", () => {
  afterAll(async () => {
    const db = OracleDatabase.getInstance();
    await db.closePool();
  });

  it("should be a singleton", () => {
    const db1 = OracleDatabase.getInstance();
    const db2 = OracleDatabase.getInstance();
    expect(db1).toBe(db2);
  });

  it("should throw error if not initialized", async () => {
    const db = OracleDatabase.getInstance();
    try {
      await db.getConnection();
      fail("Should have thrown an error");
    } catch (error) {
      expect(error).toBeDefined();
    }
  });
});
