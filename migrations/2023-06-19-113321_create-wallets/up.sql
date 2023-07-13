-- Your SQL goes here
CREATE TABLE wallets(
  `id` INTEGER AUTO_INCREMENT NOT NULL,
  `members` TEXT NOT NULL,
  `points` FLOAT NOT NULL,
  primary key(`id`)
);
