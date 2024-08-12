pub fn env_content(url: &str, port: &str, server_url: &str) -> String {
    let env = format!(
        "DATABASE_URL = {}
        HTTP_PORT = {}
        SERVER_URL = {}",
        url, port, server_url
    );

    env
}

pub fn index_content() -> String {
    let index_ts = String::from(
        "
        import express from 'express';
        import cors from 'cors';
        import morgan from 'morgan'
        import http from 'node:http';

        export const app = express()

        app.use(cors())
        app.use(morgan('dev'))
        app.use(express.json())

        app.listen(config.HTTP_PORT, () => console.log('App listening on port: ${config.HTTP_PORT}'))


        ",
    );

    index_ts
}

pub fn package_content() -> String {
    let package_json = String::from(
        r#"
        {
            "name": "demo-express",
            "version": "1.0.0",
            "main": "index.js",
            "private": true,
            "scripts": {
                "start": "node main"
            }
            "dependencies": {
                "cors": "2.8.5",
                "dotenv": "^16.4.5",
                "envalid": "8.0.0",
                "ethers": "^6.13.1",
                "express": "4.19.2",
                "morgan": "1.10.0",
            },
            "devDependencies": {
                "eslint": "8.57.0",
                "eslint-config-prettier": "9.1.0",
                "eslint-plugin-prettier": "5.1.3",
                "jest": "29.7.0",
                "prettier": "3.2.5",
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
        import * as dotenv from 'dotenv'
        import { cleanEnv, num, str, bool } from 'envalid';

        dotenv.config()

        export default cleanEnv(process.env, {
         HTTP_PORT: num({
            desc: 'Default port wher parse-server will run on',
            default: 1337,
         }),
        SERVER_URL: str({
            desc: 'Referenece to your server URL. Replace this when your app is hosted',
            devDefault: 'http://localhost:1337/server',
         }),
        })
        ",
    );

    config_ts
}
