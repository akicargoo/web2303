// 사용라이브러리 : sqlx, serde, derive_builder

use std::collections::HashMap;

use sqlx::postgres::PgPoolOptions;
use sqlx::Postgres;
use sqlx::Pool;

use serde::{Deserialize};
use derive_builder::{Builder};

pub async fn create_connection_pool(key: &str) -> Result<Pool<Postgres>, sqlx::Error> {
    let connection_info = ConnectionInfo::from_datasource_yaml(key);
    println!("{:#?}", connection_info);
    let pool = connection_info.build_pool().await?;
    Ok(pool)
}


// Postgres 커넥션 정보를 담을 구조체 정의
#[derive(Debug, Builder, Default, Deserialize, Clone)]
pub struct ConnectionInfo {
    pub host: String, // 호스트 주소
    pub port: u16, // 포트 번호
    pub username: String, // 사용자 이름
    pub password: String, // 비밀번호
    pub database: String, // 데이터베이스 이름
    pub maxsize: u32, // 최대 커넥션 수
}

impl ConnectionInfo {
    pub fn from_datasource_yaml(key: &str) -> ConnectionInfo {

        // application.yaml 파일의 경로를 지정합니다.
        let config_path = std::env::current_dir().unwrap().join("src/application.yaml");

        // application.yaml 파일을 읽어옵니다.
        let config = config::Config::builder()
                .add_source(config::File::from(config_path))
                .build()
                .unwrap();

        // application.yaml 파일에서 spring 키를 사용하여 HashMap을 가져옵니다.
        let datasource_map: HashMap<String, ConnectionInfo> = config
            .get("spring")
            .expect("Failed to get datasource configuration");

        // 여기에서 키를 사용하여 ConnectionInfo를 가져옵니다.
        let connection_info = datasource_map.get(key).expect("Failed to get connection info");

        connection_info.clone()
    
    }

    pub fn get_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }

    pub async fn build_pool(&self) -> Result<Pool<sqlx::Postgres>, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(self.maxsize)
            .connect(&self.get_url())
            .await?;
        Ok(pool)
    }

}