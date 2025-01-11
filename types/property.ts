export interface Property {
   id: string;
   kind: number;
   title: string;
   location: {
      street: string;
      city: string;
      country: string;
   };
   // Add other property fields as needed
}
