pipeline {
  agent any
  stages {
    stage('Build') {
      steps {
        sh 'cargo build'
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
            sh 'cargo +nightly fmt --all -- --check'
          }
        }
      }
    }
  }
}
