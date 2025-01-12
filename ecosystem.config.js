module.exports = {
   apps: [
      {
         name: "resin",
         script: ".output/server/index.mjs",
         instances: "max",
         exec_mode: "cluster",
         env: {
            NODE_ENV: "production",
            PORT: 3000,
         },
      },
   ],
};
