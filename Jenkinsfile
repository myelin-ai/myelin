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
    stage('rustfmt') {
      steps {
        sh 'cargo fmt --all -- --check'
      }
    }
    stage('clippy') {
      steps {
        sh 'cargo +nightly clippy'
      }
    }
  }
}
