import QRCodeStyling from "qr-code-styling";

type QRCodeOptions = {
  data: string;
  image?: string;
  width?: number;
  height?: number;
};

export default defineNuxtPlugin(() => {
  return {
    provide: {
      createQRCode: (options: QRCodeOptions): QRCodeStyling => {
        return new QRCodeStyling({
          width: options.width || 256,
          height: options.height || 256,
          data: options.data,
          type: "svg",
          dotsOptions: {
            color: "#000000",
            type: "square",
            roundSize: false
          },
          cornersSquareOptions: {
            type: "square"
          },
          cornersDotOptions: {
            type: "square"
          },
          backgroundOptions: {
            color: "#ffffff",
          },
          ...(options.image && {
            image: options.image,
            imageOptions: {
              crossOrigin: "anonymous",
            //   margin: 10,
              imageSize: 0.4,
              hideBackgroundDots: true
            }
          })
        });
      }
    }
  };
}); 