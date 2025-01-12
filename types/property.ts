export interface Property {
   id: string;
   kind: number;
   title: string;
   description: string;
   attribution?: string;
   price: number;
   location: {
      street: string;
      city: string;
      country: string;
      district?: string;
      coordinates: [number, number];
   };
   property: {
      bedrooms: number;
      size: number;
   };
   additional_details: {
      [key: string]: string | number | boolean;
   };
   key_features: string[];
   images: Array<{
      blurhash: string;
      files: Array<{
         height: number;
         width: number;
         url: string;
      }>;
   }>;
   "resin-type": "Buy Now" | "Rent to Own";
}
