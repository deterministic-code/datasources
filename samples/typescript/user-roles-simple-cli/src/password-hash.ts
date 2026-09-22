import bcrypt from "bcryptjs";

export const passwordHash = (password: string): string => bcrypt.hashSync(password, 4);
