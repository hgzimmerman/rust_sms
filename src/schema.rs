//This will codegen the import path for each table. This will create the module path: schema::users... for the users table. `diesel migration run` must me ran for this to take effect, the code will not compile otherwise.

infer_schema!("dotenv:DATABASE_URL");