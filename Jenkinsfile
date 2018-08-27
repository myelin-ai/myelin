pipeline {
  agent any
  stages {
    stage('Build') {
      steps {
        sh 'cargo build'
        sh 'bash visualization/scripts/build.sh'
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
            sh 'cargo +nightly clippy -- -Dwarnings'
          }
        }
        stage('rustfmt') {
          steps {
            sh 'cargo fmt --all -- --check'
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
