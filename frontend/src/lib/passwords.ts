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

export function genPassword(length: number): string {
    const lowerChars = 'abcdefghijklmnopqrstuvwxyz';
    const upperChars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ';
    const numChars = '0123456789';
    const specialChars = '!@#$%^&*-_.';
    const allChars = lowerChars + upperChars + numChars + specialChars;

    // Start with one of each required character
    const lowerChar = lowerChars[Math.floor(Math.random() * lowerChars.length)];
    const upperChar = upperChars[Math.floor(Math.random() * upperChars.length)];
    const numberChar = numChars[Math.floor(Math.random() * numChars.length)];
    const specialChar = specialChars[Math.floor(Math.random() * specialChars.length)];
    const password = [lowerChar, upperChar, numberChar, specialChar];

    // Fill to length
    for (let i = password.length; i < length; i++) {
        const randomIndex = Math.floor(Math.random() * allChars.length);
        password.push(allChars[randomIndex]);
    }

    // Shuffle
    // https://stackoverflow.com/questions/3943772/how-do-i-shuffle-the-characters-in-a-string-in-javascript
    for (let i = length - 1; i > 0; i--) {
        const j = Math.floor(Math.random() * (i + 1));
        const tmp = password[i];
        password[i] = password[j];
        password[j] = tmp;
    }

    return password.join('');
}
