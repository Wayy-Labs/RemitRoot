export interface User {
  id: number;
  email: string;
  firstName: string;
  lastName: string;
  createdAt: Date;
  updatedAt: Date;
}

export class UserModel {
  static tableName = "users";

  static getCreateTableSQL(): string {
    return `
      CREATE TABLE ${this.tableName} (
        id NUMBER PRIMARY KEY,
        email VARCHAR2(255) UNIQUE NOT NULL,
        first_name VARCHAR2(100) NOT NULL,
        last_name VARCHAR2(100) NOT NULL,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
      )
    `;
  }

  static getSequenceSQL(): string {
    return `
      CREATE SEQUENCE ${this.tableName}_seq
      START WITH 1
      INCREMENT BY 1
      NOCYCLE
    `;
  }
}
