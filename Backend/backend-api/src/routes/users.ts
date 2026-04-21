import { Router, Request, Response } from "express";
import { v4 as uuidv4 } from "uuid";

const router = Router();

// Mock data
const users: any[] = [];

/**
 * GET /api/users
 * Get all users
 */
router.get("/", (_req: Request, res: Response): void => {
  res.json({
    success: true,
    data: users,
    count: users.length,
  });
});

/**
 * GET /api/users/:id
 * Get user by ID
 */
router.get("/:id", (req: Request, res: Response): void => {
  const { id } = req.params;
  const user = users.find((u) => u.id === id);

  if (!user) {
    res.status(404).json({
      success: false,
      error: "User not found",
    });
    return;
  }

  res.json({
    success: true,
    data: user,
  });
});

/**
 * POST /api/users
 * Create a new user
 */
router.post("/", (req: Request, res: Response): void => {
  try {
    const { name, email } = req.body;

    if (!name || !email) {
      res.status(400).json({
        success: false,
        error: "Name and email are required",
      });
      return;
    }

    const user = {
      id: uuidv4(),
      name,
      email,
      createdAt: new Date().toISOString(),
    };

    users.push(user);

    res.status(201).json({
      success: true,
      data: user,
      message: "User created successfully",
    });
  } catch (error) {
    res.status(500).json({
      success: false,
      error: "Internal server error",
    });
  }
});

/**
 * PUT /api/users/:id
 * Update user by ID
 */
router.put("/:id", (req: Request, res: Response): void => {
  try {
    const { id } = req.params;
    const { name, email } = req.body;

    const userIndex = users.findIndex((u) => u.id === id);

    if (userIndex === -1) {
      res.status(404).json({
        success: false,
        error: "User not found",
      });
      return;
    }

    users[userIndex] = {
      ...users[userIndex],
      name: name || users[userIndex].name,
      email: email || users[userIndex].email,
      updatedAt: new Date().toISOString(),
    };

    res.json({
      success: true,
      data: users[userIndex],
      message: "User updated successfully",
    });
  } catch (error) {
    res.status(500).json({
      success: false,
      error: "Internal server error",
    });
  }
});

/**
 * DELETE /api/users/:id
 * Delete user by ID
 */
router.delete("/:id", (req: Request, res: Response): void => {
  try {
    const { id } = req.params;
    const userIndex = users.findIndex((u) => u.id === id);

    if (userIndex === -1) {
      res.status(404).json({
        success: false,
        error: "User not found",
      });
      return;
    }

    const deletedUser = users.splice(userIndex, 1);

    res.json({
      success: true,
      data: deletedUser[0],
      message: "User deleted successfully",
    });
  } catch (error) {
    res.status(500).json({
      success: false,
      error: "Internal server error",
    });
  }
});

export default router;
