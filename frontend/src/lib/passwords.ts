/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

/** Function that checks whether two passwords are the same. */
export function checkSamePassword(password: string, repeatPassword: string): boolean {
    return repeatPassword == password;
}

/** Function to check whether a password contain a lowercase character. */
export function checkLowercase(password: string): boolean {
    const passwordChecker = password.toUpperCase();
    return passwordChecker != password;
}

/** Function to check whether a password contains an uppercase character.  */
export function checkUppercase(password: string): boolean {
    const passwordChecker = password.toLowerCase();
    return passwordChecker != password;
}

/** Function to check whether a password contains a number. */
export function checkNumber(password: string): boolean {
    const regex = /\d/;
    return regex.test(password);
}

/** Function to check whether a password contains a special character. */
export function checkSpecialCharacter(password: string): boolean {
    const regex = /[?!@#$%^&*()+=.,<>{}[\]:;"'|~`_/\\-]/;
    return regex.test(password);
}
