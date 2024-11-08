type JsonValue =
   | string
   | number
   | boolean
   | null
   | JsonValue[]
   | { [key: string]: JsonValue };

/**
 * Fixes nested JSON strings by parsing them into their proper object form
 * @param obj - The object containing potential JSON strings that need to be parsed
 * @returns The same object with any JSON strings parsed into their proper form
 */
export const fixNestedStrings = <T extends Record<string, unknown>>(
   obj: T,
): T => {
   // Iterate over each key in the object
   for (const key in obj) {
      if (Object.prototype.hasOwnProperty.call(obj, key)) {
         const value = obj[key];

         if (typeof value === "string") {
            try {
               // Try to parse the string as JSON
               const parsedValue = JSON.parse(value) as JsonValue;
               // If successful, replace the string with the parsed object or array
               obj[key] = parsedValue as T[typeof key];
            } catch (e) {
               // If parsing fails, keep the original string value
               console.error(e);
               continue;
            }
         }
      }
   }

   return obj;
};

/* Example usage:
interface UserData {
   name: string;
   preferences: string | Record<string, unknown>;
   metadata: string | Record<string, unknown>;
}

The function can be used like this:
 const data: UserData = {
   name: "John",
   preferences: '{"theme":"dark","notifications":true}',
   metadata: '{"lastLogin":"2024-01-01"}'
 };
 const fixed = fixNestedStrings(data);
*/
