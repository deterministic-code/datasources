DROP PROCEDURE IF EXISTS add_user;
GO
CREATE PROCEDURE add_user(IN p_username TEXT, IN p_password TEXT, IN p_is_enabled TINYINT)
INSERT INTO `users` (username, password, is_enabled)
VALUES (p_username, p_password, p_is_enabled);
GO
DROP PROCEDURE IF EXISTS add_role;
GO
CREATE PROCEDURE add_role(IN p_name TEXT, IN p_is_enabled TINYINT)
INSERT INTO `roles` (name, is_enabled)
VALUES (p_name, p_is_enabled);
GO
DROP PROCEDURE IF EXISTS add_group;
GO
CREATE PROCEDURE add_group(IN p_name TEXT, IN p_is_enabled TINYINT)
INSERT INTO `groups` (ugly_name, is_enabled)
VALUES (p_name, p_is_enabled);
GO
DROP PROCEDURE IF EXISTS add_user_role;
GO
CREATE PROCEDURE add_user_role(IN p_user_id INT, IN p_role_id INT)
INSERT INTO `user_roles` (user_id, bad_role_id)
VALUES (p_user_id, p_role_id);
GO
DROP PROCEDURE IF EXISTS add_user_group;
GO
CREATE PROCEDURE add_user_group(IN p_user_id INT, IN p_group_id INT)
INSERT INTO `user_groups` (user_id, group_id)
VALUES (p_user_id, p_group_id);
GO
DROP PROCEDURE IF EXISTS set_user_enabled;
GO
CREATE PROCEDURE set_user_enabled(IN p_username TEXT, IN p_is_enabled TINYINT)
UPDATE `users` SET is_enabled = p_is_enabled WHERE username = p_username;
