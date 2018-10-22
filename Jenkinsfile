pipeline {
  agent any
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
        stage('yarn') {
          sh 'yarn'
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
            anyOf {
              branch 'master'
              changeRequest()
            }
          }
          steps {
            sh 'cargo doc --no-deps'
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
        stage('rustfmt') {
          steps {
            sh 'cargo fmt --all -- --check'
          }
        }
        stage('tslint') {
          steps {
            sh '(cd visualization && yarn lint)'
          }
        }
      }
    }
    stage('Deploy') {
      when {
        branch 'master'
      }
      steps {
        sh "cp -r target/doc/* ${env.DOCS_PUBLIC_PATH}"
        sh "cp docs/* ${env.DOCS_PUBLIC_PATH}"
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
