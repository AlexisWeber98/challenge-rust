pub fn parse_env(
    url: &str,
    app_id: &str,
    master_key: &str,
    port: &str,
    server_url: &str,
) -> String {
    let env = format!(
        "DATABASe_URL = {}
APPLICATION_ID = {}
MASTER_KEY = {}
HTTP_PORT = {}
SERVER_URL = {}",
        url, app_id, master_key, port, server_url
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
        
        
        export const app = express();
        
        Parse.initialize(config.APPLICATION_ID, config.MASTER_KEY);
        
        
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
         
        }
        ",
    );
    config_ts
}

pub fn parse_server_content() -> String {
    let parse_server = format!("parse");

    parse_server
}
