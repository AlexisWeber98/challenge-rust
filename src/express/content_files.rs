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
        import morgan from 'morgan';
        import http from 'node:http';
        import config from './config.js';
        import routes from './routes/index.js'

        export const app = express();

        app.use(cors());
        app.use(morgan('dev'));
        app.use(express.json());

        app.use('/api', routes)

        app.listen(config.HTTP_PORT, () =>
          console.log(`App listening on port: ${config.HTTP_PORT}`),
        );

        ",
    );

    index_ts
}

pub fn routes_create() -> String {
    let routes_index = String::from(
        "
        import { Router } from 'express';
        import userRouter from '../modules/user/userRouter.js';

        const routes = Router();

        routes.use('/users', userRouter);

        export default routes;
        ",
    );
    routes_index
}

pub fn user_controllers() -> String {
    let user_controller = String::from(
        "
            import {
              createUser,
              getUserById,
              getUsers,
              putUser,
              deleteUser,
            } from '../services/userServices.js';

            export const GetUsers = async (_req, res) => {
              const users = await getUsers();
              res.status(users.status).json({ response: users.response });
            };

            export const GetUserById = async (req, res) => {
              const { id } = req.params;
              const user = await getUserById(id);
              res.status(user.status).json({ response: user.response });
            };

            export const CreateUser = async (req, res) => {
              const { body } = req;
              const newUser = await createUser(body);
              res.status(newUser.status).json({ response: newUser.response });
            };

            export const PutUser = async (req, res) => {
              const { id } = req.params;
              const { body } = req;
              const updatedUser = await putUser(id, body);
              res.status(updatedUser.status).json({ response: updatedUser.response });
            };

            export const DeleteUser = async (req, res) => {
              const { id } = req.params;
              const deletedUser = await deleteUser(id);
              res.status(deletedUser.status).json({ response: deleteUser.response });
            };
        ",
    );
    user_controller
}

pub fn user_services() -> String {
    let user_service = String::from(
        "
        export const getUsers = async () => {
          try {
            return {
              status: 200,
              response: 'Hola desde getUsers',
            };
          } catch (error) {
            throw error;
          }
        };

        export const getUserById = async (id) => {
          try {
            return {
              status: 200,
              response: `Hola desde getUserById con id: ${id}`,
            };
          } catch (error) {
            throw error;
          }
        };

        export const createUser = async (body) => {
          try {
            return {
              status: 300,
              response: `Hola desde createUser con body: ${body}`,
            };
          } catch (error) {
            throw error;
          }
        };

        export const putUser = async (id, body) => {
          try {
            return {
              status: 200,
              response: `Hola desde putUser con id: ${id} y body: ${body}`,
            };
          } catch (error) {
            throw error;
          }
        };

        export const deleteUser = async (id) => {
          try {
            return {
              status: 400,
              response: `Adios al usuario con id: ${id}`,
            };
          } catch (error) {
            throw error;
          }
        };

        ",
    );
    user_service
}

pub fn user_router() -> String {
    let user_route = String::from(
        "
        import { Router } from 'express';
        import {
          CreateUser,
          DeleteUser,
          GetUserById,
          GetUsers,
          PutUser,
        } from './controllers/userControllers.js';

        const userRouter = Router();

        userRouter.get('/', GetUsers);
        userRouter.get('/:id', GetUserById);
        userRouter.post('/', CreateUser);
        userRouter.put('/:id', PutUser);
        userRouter.delete('/:id', DeleteUser);

        export default userRouter;
        ",
    );
    user_route
}

pub fn package_content() -> String {
    let package_json = String::from(
        r#"
        {
          "name": "demo-express",
          "version": "1.0.0",
          "main": "index.js",
          "private": true,
          "type": "module",
          "scripts": {
            "start": "node src/index.js"
          },
          "dependencies": {
            "cors": "2.8.5",
            "dotenv": "^16.4.5",
            "envalid": "8.0.0",
            "ethers": "^6.13.1",
            "express": "4.19.2",
            "morgan": "1.10.0"
          },
          "devDependencies": {
            "eslint": "8.57.0",
            "eslint-config-prettier": "9.1.0",
            "eslint-plugin-prettier": "5.1.3",
            "jest": "29.7.0",
            "prettier": "3.2.5"
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
