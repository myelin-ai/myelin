pipeline {
  agent any
  stages {
    stage('Build') {
      parallel {
        stage('cargo') {
          steps {
            sh 'cargo build'
          }
        }
        stage('wasm') {
          steps {
            sh 'bash visualization/scripts/build.sh'
          }
        }
      }
    }
    stage('Test') {
      steps {
        sh 'cargo test'
      }
    }
    stage('Style checks') {
      parallel {
        stage('clippy') {
          steps {
            sh 'cargo clippy -- -Dwarnings'
          }
        }
        stage('rustfmt') {
          steps {
            sh 'cargo fmt --all -- --check'
          }
        }
        stage('tslint') {
          steps {
            sh '(cd visualization && tslint --project tsconfig.json \'src/**/*.ts\')'
          }
        }
      }
    }
  }
  post {
    failure {
      step([$class: 'TelegramBotPublisher', message: 'Branch ${BUILD_TAG} failed. ${RUN_DISPLAY_URL}', whenFailed: true])
    }
  }
}
