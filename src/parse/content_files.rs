pub fn parse_env(
    url: &str,
    app_id: &str,
    master_key: &str,
    port: &str,
    server_url: &str,
    project_name: &str,
) -> String {
    let env = format!(
"DATABASE_URL = {}
APPLICATION_ID = {}
MASTER_KEY = {}
HTTP_PORT = {}
SERVER_URL = {}
APP_NAME = {}
CLOUD_PATH = './build/cloud/main.js'
",
        url, app_id, master_key, port, server_url, project_name
    );

    env
}

pub fn index_content() -> String {
    let index_ts = String::from(
        "
        import express from 'express';
        import ParseServer from 'parse-server';
        import { parseServer } from './parseServer';
        import cors from 'cors';
        import http from 'node:http';
        import config from './config';
        
        
        export const app = express();
        
        Parse.initialize(config.APPLICATION_ID, config.MASTER_KEY);
        Parse.serverURL = config.SERVER_URL;
        
        app.use(express.json());
        app.use(cors());
        app.use(`/api`, parseServer.app);
        
        
        export const httpServer = http.createServer(app)
        
        httpServer.listen(config.HTTP_PORT, async () => {
          
          return config.HTTP_PORT;
        })
        
        ParseServer.createLiveQueryServer(httpServer);
        ",
    );

    index_ts
}

pub fn package_content() -> String {
    let package_json = String::from(
        r#"
        {
        "name": "demo-parse-server",
          "version": "1.0.0",
          "main": "dist/index.js",
          "private": true,
          "dependencies": {
            "cors": "2.8.5",
            "dotenv": "^16.4.5",
            "envalid": "8.0.0",
            "ethers": "^6.13.1",
            "express": "4.19.2",
            "parse-dashboard": "5.4.0",
            "parse-server": "7.0.0"
          },
          "devDependencies": {
            "@babel/core": "^7.24.7",
            "@babel/preset-env": "^7.24.7",
            "@babel/preset-typescript": "^7.24.7",
            "@types/cors": "2.8.17",
            "@types/node": "20.14.2",
            "@types/parse": "^3.0.9",
            "@typescript-eslint/eslint-plugin": "7.13.0",
            "@typescript-eslint/parser": "7.13.0",
            "eslint": "9.4.0",
            "eslint-plugin-etc": "2.0.3",
            "eslint-plugin-import": "2.29.1",
            "mongodb-runner": "5.6.2",
            "nodemon": "^3.1.4",
            "prettier": "3.3.2",
            "ts-jest": "^29.1.5",
            "ts-node": "10.9.2",
            "typescript": "5.4.5"
          },
          "scripts": {
            "dev": "npx nodemon",
            "build": "tsc",
            "start": "node ./build/index.js",
            "gen:lockfile": "yarn install --mode update-lockfile",
            "lint": "eslint --ext .js,.ts .",
            "format": "prettier --write 'src/**/*.{js,ts}'",
            "dev:db-start": "mongodb-runner start",
            "dev:db-stop": "mongodb-runner stop",
            "createAll": "node src/utils/create/index.create.js"
          },
          "engines": {
            "node": "18.19.1"
          }
        }
    "#,
    );

    package_json
}

pub fn config_content() -> String {
    let config_ts = String::from(
        "
        import * as dotenv from 'dotenv';
        import { cleanEnv, num, str, bool } from 'envalid';
        
        dotenv.config();
        
        export default cleanEnv(process.env, {
        
        MASTER_KEY: str({
           desc: 'A secret key of your choice (keep this secret)',
         }),
         APPLICATION_ID: str({
           desc: 'An id for your app, can be anything you want',
           default: 'APPLICATION_ID',
         }),
         HTTP_PORT: num({
            desc: 'Default port wher parse-server will run on',
            default: 1337,
         }),
        SERVER_URL: str({
            desc: 'Referenece to your server URL. Replace this when your app is hosted',
            devDefault: 'http://localhost:1337/server',
         }),
         DATABASE_URL: str({
         desc: 'url database'
         }),
         APP_NAME: str({
             desc: 'app name'
         }),
         CLOUD_PATH: str({
             desc: 'path cloud main'
         })
        })
        ",
    );
    config_ts
}

pub fn parse_server_content() -> String {
    let parse_server = String::from(
        r#"
        // @ts-ignore
        import { ParseServer } from 'parse-server';
        import config from './config';
        
        const dataBaseUri =  config.DATABASE_URL;
        
        const serverPublicUrl = config.SERVER_URL;
        
        export const parseServer = new ParseServer({
          liveQuery: {
            classNames: ["ItemsMinted", "CollectionsPolygon"]
          },
          websocketTimeout: 60 * 1000,
        
          databaseURI: dataBaseUri,
          cloud: config.CLOUD_PATH,
          serverURL: config.SERVER_URL,
          logsFolder: './logs',
          publicServerURL: serverPublicUrl,
          appName: config.APP_NAME,
          appId: config.APPLICATION_ID,
          masterKey: config.MASTER_KEY,
          allowClientClassCreation: true,
          verifyUserEmails: false,
          emailVerifyTokenValidityDuration: 2 * 60 * 60,
        });
        
        (() => {
          parseServer.start();
        })();
        
        "#,
    );

    parse_server
}

pub fn tsconfig_content() -> String {
    let tsconfig = String::from(
        r#"
        {
          "compilerOptions": {
            "strict": true,
            "module": "commonjs",
            "outDir": "build",
            "declaration": true,
            "declarationDir": "build",
            "sourceMap": true,
            "target": "es2017",
            "rootDir": ".",
            "esModuleInterop": true,
            "skipLibCheck": true,
            "allowJs": true
          },
          "include": ["src", "cloud"],
          "files": ["src/global.d.ts"]
        }

        "#,
    );

    tsconfig
}

pub fn nodemon_content() -> String {
    let nodemon_json = String::from(
        r#"{
      "watch": ["src", "cloud"],
      "ext": "ts js",
      "exec": "yarn build && ts-node ./src/index.ts"
    }"#,
    );

    nodemon_json
}

pub fn main_content() -> String {
    let main_js = String::from("require ('./routes/index.routes')");

    main_js
}

pub fn routs_content() -> String {
    let  index_routes_js = String::from(
        r#"
        const path = require('path');
        const fs = require('fs');
        
        const PATH_NAME = path.join(__dirname, '../modules/');
        
        fs.readdirSync(PATH_NAME).forEach((folder) => {
          const folderPath = path.join(PATH_NAME, folder);
          const files = fs.readdirSync(folderPath);
          files.forEach((file) => {
            if (file.endsWith('Clouds.js')) {
              const filePath = path.join(folderPath, file);
              require(filePath);
            }
          });
        });
        "#,
    );

    index_routes_js
}

pub fn user_clouds_content() -> String {
    let user_clouds_js = String::from(r#"
        import { createUserNewControllers, deleteUserControllers, getAllUsersControllers, getUserByIdControllers, updateUserControllers } from './userControllers';
        
        
        Parse.Cloud.define('getAllUsers', getAllUsersControllers(Parse));
        Parse.Cloud.define('getUserById', getUserByIdControllers(Parse), { requireUser: true });
        Parse.Cloud.define('createUser', createUserNewControllers(Parse));
        Parse.Cloud.define('deleteUser', deleteUserControllers(Parse), { requireUser: true });
        Parse.Cloud.define('updateUser', updateUserControllers(Parse), { requireUser: true });"#);
    
    user_clouds_js
}

pub fn user_controller_content() -> String {
    let user_controller_js = String::from (r#"
        import { 
          createNewUserServices, 
          deleteUserServices, 
          getAllUsersServices, 
          getUserByIdServices, 
          updateUserServices 
        } from "./userServices";
        
        const handleRequest = (serviceFunction) => {
          return async (request) => {
            try {
              const result = await serviceFunction(request.params, request.headers);
              return {
                
                status_result: "Ok",
                ...result,
              };
            } catch (error) {
              console.error(`Error code: ${error.code}, Error message: ${error.message}`);
              return {
                status: 'error',
                result: false,
                errorDetails: {
                  code: error.code || Parse.Error.INTERNAL_SERVER_ERROR,
                  message: error.message,
                },
              };
            }
          };
        };
        
        export function getAllUsersControllers(Parse) {
          return handleRequest(async ({ page }) => {
            const users = await getAllUsersServices(page);
            return { users };
          });
        }
        
        export function getUserByIdControllers(Parse) {
          return handleRequest(async ({ userId }) => {
            const user = await getUserByIdServices(userId);
            return { user };
          });
        }
        
        export function createUserNewControllers(Parse) {
          return handleRequest(async ({ objectData }) => {
            const user = await createNewUserServices(objectData);
            return { user };
          });
        }
        
        export function updateUserControllers(Parse) {
          return handleRequest(async ({ userId, objectData }) => {
            const user = await updateUserServices(userId, objectData);
            return { user };
          });
        }
        
        export function deleteUserControllers(Parse) {
          return handleRequest(async ({ userId }, headers) => {
            const sessionToken = headers['x-parse-session-token'];
            await deleteUserServices(userId, sessionToken);
            return { message: 'User deleted successfully' };
          });
        }
        "#);
    
    user_controller_js
}

pub fn user_srvices_content() -> String {
    let user_services_js = String::from(r#"
        const User = Parse.Object.extend('User');
        
        // Helper function to handle errors
        function throwError(errorCode, errorMessage) {
          throw new Parse.Error(errorCode, errorMessage);
        }
        
        // Helper function to fetch a user by ID
        async function fetchUserById(userId) {
          const query = new Parse.Query(User);
          query.equalTo('objectId', userId);
          return await query.first({ useMasterKey: true });
        }
        
        export async function getAllUsersServices(page) {
          if (page === null || page === undefined) {
            throwError(Parse.Error.INVALID_QUERY, 'Page number is missing.');
          }
        
          const query = new Parse.Query(User);
          const users = await query.find({ useMasterKey: true });
        
          if (!users || users.length === 0) {
            throwError(Parse.Error.OBJECT_NOT_FOUND, 'No users found.');
          }
        
          return users;
        }
        
        export async function getUserByIdServices(userId) {
          if (!userId) {
            throwError(Parse.Error.INVALID_QUERY, 'User ID is missing.');
          }
        
          const user = await fetchUserById(userId);
          if (!user) {
            throwError(Parse.Error.OBJECT_NOT_FOUND, `User with ID ${userId} does not exist.`);
          }
        
          return user;
        }
        
        export async function createNewUserServices(objectData) {
          if (!objectData) {
            throwError(Parse.Error.OBJECT_NOT_FOUND, 'objectData is missing.');
          }
        
          const user = new User();
          Object.keys(objectData).forEach(key => {
            if (objectData.hasOwnProperty(key)) {
              user.set(key, objectData[key]);
            }
          });
        
          await user.save();
          return user;
        }
        
        export async function updateUserServices(userId, objectData) {
          if (!userId || !objectData) {
            throwError(Parse.Error.INVALID_QUERY, 'User ID or objectData is missing.');
          }
        
          const user = await fetchUserById(userId);
          if (!user) {
            throwError(Parse.Error.OBJECT_NOT_FOUND, `User with ID ${userId} does not exist.`);
          }
        
          const nonUpdatableFields = ['username', 'email', 'password'];
          Object.keys(objectData).forEach(key => {
            if (!nonUpdatableFields.includes(key)) {
              user.set(key, objectData[key]);
            }
          });
        
          await user.save(null, { useMasterKey: true });
          return user;
        }
        
        export async function deleteUserServices(userId) {
          if (!userId) {
            throwError(Parse.Error.INVALID_QUERY, 'User ID is missing.');
          }
        
          const user = await fetchUserById(userId);
          if (!user) {
            throwError(Parse.Error.OBJECT_NOT_FOUND, `User with ID ${userId} does not exist.`);
          }
        
          await user.destroy({ useMasterKey: true });
        }

        "#);
    user_services_js
}
