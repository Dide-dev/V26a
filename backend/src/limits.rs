/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

// Card data limits
pub const CARD_FRONT_MAX_LENGTH: usize = 250;
pub const CARD_BACK_MAX_LENGTH: usize = 5000;
pub const CARD_LIBRARY_TITLE_MAX_LENGTH: usize = 100;
pub const CARD_LIBRARY_TAG_MAX_LENGTH: usize = 32;
// Min
pub const CARD_BACK_MIN_LENGTH: usize = 1;
pub const CARD_LIBRARY_TITLE_MIN_LENGTH: usize = 1;
pub const CARD_LIBRARY_TAG_MIN_LENGTH: usize = 2;

// Classroom data limits
pub const CLASSROOM_TITLE_MAX_LENGTH: usize = 32;
pub const CLASSROOM_DESCRIPTION_MAX_LENGTH: usize = 256;
pub const LESSON_TITLE_MAX_LENGTH: usize = 32;
pub const LESSON_DESCRIPTION_MAX_LENGTH: usize = 128;
// Min
pub const CLASSROOM_TITLE_MIN_LENGTH: usize = 1;

// Credential data limits
pub const EMAIL_MAX_LENGTH: usize = 128;
pub const PASSWORD_MAX_LENGTH: usize = 72;
// Minimums
pub const PASSWORD_MIN_LENGTH: usize = 8;
pub const PASSWORD_LOGIN_MIN_LENGTH: usize = 1; // To accommodate old accounts with a password shorter than the current imposed minimum.

// Feedback data limits
pub const FEEDBACK_MAX_LENGTH: usize = 5000;
// Minimums
pub const FEEDBACK_MIN_LENGTH: usize = 6;

// Tree data limits
pub const TREE_TITLE_MAX_LENGTH: usize = 100;
pub const TREE_DESCRIPTION_MAX_LENGTH: usize = 500;
pub const TREE_BOOK_NAME_MAX_LENGTH: usize = 64;
pub const TREE_BOOK_EDITION_MAX_LENGTH: usize = 32;
pub const TREE_BOOK_ISBN_MAX_LENGTH: usize = 17;
pub const TREE_BOOK_CHAPTER_MAX_LENGTH: usize = 32;
pub const TREE_BOOK_EXERCISE_MAX_LENGTH: usize = 32;
pub const TREE_ASSIGNMENT_MAX_LENGTH: usize = 5000;
pub const TREE_TAG_MAX_LENGTH: usize = 32;
// Minimums
pub const TREE_TITLE_MIN_LENGTH: usize = 4;
pub const TREE_BOOK_ISBN_MIN_LENGTH: usize = 10;
pub const TREE_TAG_MIN_LENGTH: usize = 2;

// User data limits
pub const USER_NAME_MAX_LENGTH: usize = 32;
pub const USER_NICKNAME_MAX_LENGTH: usize = 32;
pub const USER_ABOUT_ME_MAX_LENGTH: usize = 512;
// Minimums
pub const USER_NAME_MIN_LENGTH: usize = 4;
pub const USER_NICKNAME_MIN_LENGTH: usize = 2;
