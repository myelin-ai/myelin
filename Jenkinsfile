pipeline {
  agent any
  options {
    disableConcurrentBuilds()
  }
  stages {
    stage('Clean') {
      when {
        anyOf {
          branch 'master'
          changeRequest()
        }
      }
      steps {
        sh 'git checkout .'
        sh 'git clean -xfd'
      }
    }
    stage('Dependencies') {
      parallel {
        stage('rustup') {
          steps {
            sh './scripts/post-rustup-install.sh'
          }
        }
        stage('yarn') {
          steps {
            sh '(cd visualization-client && yarn)'
          }
        }
        stage('poetry') {
          steps {
            sh '(cd docs && poetry install)'
          }
        }
      }
    }
    stage('Build') {
      parallel {
        stage('cargo build') {
          steps {
            sh 'cargo build'
          }
        }
        stage('cargo doc') {
          when {
            branch 'master'
          }
          steps {
            sh 'cargo doc --all --all-features'
            sh '(cd docs && poetry run ./build-index.py)'
          }
        }
        stage('cargo doc --no-deps') {
          when {
            changeRequest()
          }
          steps {
            sh 'cargo doc --all --all-features --no-deps'
            sh '(cd docs && poetry run ./build-index.py)'
          }
        }
        stage('wasm') {
          steps {
            sh './visualization-client/scripts/build.py --webpack'
          }
        }
      }
    }
    stage('Test') {
      parallel {
        stage('cargo test') {
          steps {
            sh 'cargo test --all-features'
          }
        }
        stage('wasm-loading-test') {
          steps {
            sh 'bash visualization-client/scripts/wasm-loading-test.sh'
          }
        }
      }
    }
    stage('Style checks') {
      when {
        anyOf {
          branch 'master'
          changeRequest()
        }
      }
      parallel {
        stage('clippy') {
          steps {
            sh 'cargo clippy -- -Dwarnings'
          }
        }
        stage('clippy tests') {
          steps {
            sh 'cargo clippy --tests --all-features -- -Dwarnings'
          }
        }
        stage('rustfmt') {
          steps {
            sh 'cargo fmt --all -- --check'
          }
        }
        stage('additional lints') {
          steps {
            sh './scripts/additional-lints.py'
          }
        }
        stage('tslint') {
          steps {
            sh '(cd visualization-client && yarn lint)'
          }
        }
      }
    }
    stage('Deploy') {
      when {
        branch 'master'
      }
      steps {
        sh "tar -cvf docs.tar.gz -C target/doc ."
        sh "./.jenkins/deploy-docs.sh"
      }
    }
  }
  post {
    failure {
      script {
        if (env.BRANCH_NAME == 'master') {
          step([$class: 'TelegramBotPublisher', message: 'Branch ${BUILD_TAG} failed. ${RUN_DISPLAY_URL}', whenFailed: true])
        }
      }
    }
  }
}
