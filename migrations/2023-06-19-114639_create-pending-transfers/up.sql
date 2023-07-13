-- Your SQL goes here
CREATE TABLE pending_transfers (
  `id` INTEGER AUTO_INCREMENT NOT NULL,
  `sender_user_id` VARCHAR(191) NOT NULL,
  `receiver_user_id` VARCHAR(191) NOT NULL,
  `points` FLOAT NOT NULL
  primary key(`id`)
);
