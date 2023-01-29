CREATE TABLE IF NOT EXISTS `users` (
    `id` int NOT NULL PRIMARY KEY auto_increment,
    `email` varchar(128) UNIQUE NOT NULL,
    `password` varchar(512) NOT NULL,
    `firstName` varchar(50),
    `lastName`varchar (50),
    `birthday` date,
    `isAdmin` boolean DEFAULT false NOT NULL,
    `section` enum('Sopran', 'Alt', 'Tenor', 'Bass', 'Dirigent', 'Instrument') NOT NULL,
    `imageFilePath` varchar(200) NOT NULL,
    `street` varchar(200),
    `number` varchar(10),
    `zipCode` varchar(10),
    `city` varchar(50),
    `country` varchar(50),
    `registrationDate` date NOT NULL,
    `numberPrivate`varchar(30),
    `numberMobile`varchar(30),
    `numberBusiness`varchar(30),
    `isActive` boolean DEFAULT true NOT NULL,
    `channelEventReminder` ENUM('none', 'email', 'push', 'telegram') DEFAULT 'email' NOT NULL,
    `channelEventChange` ENUM('none', 'email', 'push', 'telegram') DEFAULT 'email' NOT NULL,
    `channelSongbookChange` ENUM('none', 'email', 'push', 'telegram') DEFAULT 'none' NOT NULL,
    `channelAlbumChange` ENUM('none', 'email', 'push', 'telegram') DEFAULT 'none' NOT NULL,
    `passwordResetKey` varchar(64) default NULL,
    `passwordResetDate` date default NULL
    );

CREATE TABLE IF NOT EXISTS `events` (
    `id` int NOT NULL PRIMARY KEY auto_increment,
    `name` varchar(100) NOT NULL,
    `description` text,
    `date` date NOT NULL,
    `start` time NOT NULL,
    `dateEnd` date DEFAULT NULL,
    `end` time NOT NULL,
    `location` varchar(200) DEFAULT NULL,
    `isRegularPractice` boolean DEFAULT true
    );

CREATE TABLE IF NOT EXISTS `userEvents` (
    `id` int NOT NULL PRIMARY KEY auto_increment,
    `userId` int NOT NULL,
    `eventId` int NOT NULL,
    `response` ENUM('yes', 'no', 'maybe'),
    `comment` varchar(100),
    FOREIGN KEY(userId) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY(eventId) REFERENCES events(id) ON DELETE CASCADE,
    UNIQUE (userId, eventId)
    );

CREATE TABLE IF NOT EXISTS `userPushConfig` (
    `id` int NOT NULL PRIMARY KEY auto_increment,
    `userId` int NOT NULL,
    `deviceName` varchar(100) NOT NULL,
    `date` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `pushConfig` varchar(500) NOT NULL UNIQUE,
    FOREIGN KEY(userId) REFERENCES users(id) ON DELETE CASCADE,
    UNIQUE (userId, deviceName)
    );

CREATE TABLE IF NOT EXISTS `blogPosts` (
  `id` int NOT NULL PRIMARY KEY auto_increment,
  `authorId` int,
  `createdAt` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updatedAt` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `header` varchar(1000),
  `content` text,
  FOREIGN KEY(authorId) REFERENCES users(id) ON DELETE SET NULL
);

INSERT INTO `users` (`email`, `password`, `firstName`, `lastName`, `birthday`, `isAdmin`, `section`, `imageFilePath`, `registrationDate`) VALUES ('test@test.com', '$6$rounds=5000$26fb6ee1f1e483d5$pH32.NS1UbRzSjvhLWhGJ2WFllu51szmiRG9WhGnHMXmOHzLrYVYfbd0AZGMUa6QZNSYGh3GRIxAew6XaXR7i1', 'Maxime', 'Mustermann', '2000-01-01', true, 'Sopran', 'defaultPortraitSopran.jpeg', '2019-06-01');
INSERT INTO `users` (`email`, `password`, `firstName`, `lastName`, `birthday`, `isAdmin`, `section`, `imageFilePath`, `registrationDate`) VALUES ('morian@test.com', '$6$rounds=5000$26fb6ee1f1e483d5$pH32.NS1UbRzSjvhLWhGJ2WFllu51szmiRG9WhGnHMXmOHzLrYVYfbd0AZGMUa6QZNSYGh3GRIxAew6XaXR7i1', 'Morian', 'Flyer', '2001-05-25', false, 'Tenor', 'defaultPortraitTenor.jpeg', '2018-01-01');
INSERT INTO `events` (`name`, `date`, `start`, `end`, `isRegularPractice`) VALUES ('Testkonzert', '2020-10-08', '01:02:03', '04:05:06', false);
INSERT INTO `userEvents` (`userId`, `eventId`, `response`, `comment`) values ('1', '1', 'yes', 'Ich freue mich drauf');