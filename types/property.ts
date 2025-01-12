export type Property = {
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

export type PropertyImage = Property["images"][number];
export type PropertyImageFile = PropertyImage["files"][number];

export const propertyImageUtils = {
   /**
    * Get the smallest image from a property's image array by width
    */
   getSmallestImage: (images: Property["images"][number]["files"]) => {
      if (!images?.length) return null;
      return images.reduce((smallest, current) => 
         current.width < smallest.width ? current : smallest
      );
   },

   /**
    * Get the largest image from a property's image array by width
    */
   getLargestImage: (images: Property["images"][number]["files"]) => {
      if (!images?.length) return null;
      return images.reduce((largest, current) => 
         current.width > largest.width ? current : largest
      );
   },

   /**
    * Get an image closest to a target width
    */
   getImageByWidth: (images: Property["images"][number]["files"], targetWidth: number) => {
      if (!images?.length) return null;
      return images.reduce((closest, current) => {
         const currentDiff = Math.abs(current.width - targetWidth);
         const closestDiff = Math.abs(closest.width - targetWidth);
         return currentDiff < closestDiff ? current : closest;
      });
   },

   /**
    * Get a srcset string from image files
    */
   getSrcSet: (images: Property["images"][number]["files"]) => {
      if (!images?.length) return "";
      return images
         .map((file) => `${file.url} ${file.width}w`)
         .join(", ");
   },

   /**
    * Get the first image from a property that matches the width exactly, or the closest match
    */
   getPropertyImageByWidth: (property: Property, width: number, index: number = 0) => {
      if (!property?.images?.[index]?.files) return null;
      
      // Try to find exact match first
      const exactMatch = property.images[index].files.find(file => file.width === width);
      if (exactMatch) return exactMatch;

      // Otherwise get closest match
      return propertyImageUtils.getImageByWidth(property.images[index].files, width);
   },

   /**
    * Get blurhash for a specific image index
    */
   getBlurhash: (property: Property, index: number = 0) => {
      return property?.images?.[index]?.blurhash || null;
   }
};
