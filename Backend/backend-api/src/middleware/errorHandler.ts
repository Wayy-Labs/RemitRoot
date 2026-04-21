import { Request, Response, NextFunction } from "express";

interface CustomError extends Error {
  status?: number;
}

export const errorHandler = (
  err: CustomError,
  _req: Request,
  res: Response,
  _next: NextFunction,
) => {
  const status = err.status || 500;
  const message = err.message || "Internal Server Error";

  console.error(`[ERROR] ${status}: ${message}`);

  res.status(status).json({
    success: false,
    error: message,
    status,
    ...(process.env.NODE_ENV === "development" && { stack: err.stack }),
  });
};
