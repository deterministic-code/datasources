DROP PROCEDURE IF EXISTS add_user;
CREATE PROCEDURE add_user(IN p_username TEXT, IN p_password TEXT, IN p_is_enabled TINYINT, IN p_created TEXT, IN p_updated BLOB)
INSERT INTO `users` (username, password, is_enabled, created, updated)
VALUES (p_username, p_password, p_is_enabled, p_created, p_updated);
DROP PROCEDURE IF EXISTS add_role;
CREATE PROCEDURE add_role(IN p_name TEXT, IN p_is_enabled TINYINT, IN p_created TEXT, IN p_updated BLOB)
INSERT INTO `roles` (name, is_enabled, created, updated)
VALUES (p_name, p_is_enabled, p_created, p_updated);
DROP PROCEDURE IF EXISTS add_group;
CREATE PROCEDURE add_group(IN p_name TEXT, IN p_is_enabled TINYINT, IN p_created TEXT, IN p_updated BLOB)
INSERT INTO `groups` (ugly_name, is_enabled, created, updated)
VALUES (p_name, p_is_enabled, p_created, p_updated);
DROP PROCEDURE IF EXISTS add_user_role;
CREATE PROCEDURE add_user_role(IN p_user_id INT, IN p_role_id INT, IN p_created TEXT, IN p_updated BLOB)
INSERT INTO `user_roles` (user_id, bad_role_id, created, updated)
VALUES (p_user_id, p_role_id, p_created, p_updated);
DROP PROCEDURE IF EXISTS add_user_group;
CREATE PROCEDURE add_user_group(IN p_user_id INT, IN p_group_id INT, IN p_created TEXT, IN p_updated BLOB)
INSERT INTO `user_groups` (user_id, group_id, created, updated)
VALUES (p_user_id, p_group_id, p_created, p_updated);
DROP PROCEDURE IF EXISTS set_user_enabled;
CREATE PROCEDURE set_user_enabled(IN p_username TEXT, IN p_is_enabled TINYINT, IN p_updated BLOB)
UPDATE `users` SET is_enabled = p_is_enabled, updated = p_updated WHERE username = p_username;
