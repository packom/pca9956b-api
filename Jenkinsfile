// https://www.jenkins.io/doc/book/pipeline/docker/
version = ''
diff_size = ''
pipeline {
    agent {
        docker { image 'piersfinlayson/openapi-gen-amd64:0.0.1' }
    }
    stages {
        stage('Clone') {
            steps {
                withCredentials([usernamePassword(credentialsId: 'github.packom', usernameVariable: 'USERNAME', passwordVariable: 'PASSWORD')]) {
                    sh '''
                        cd ~/builds && \
                        git clone https://packom:$PASSWORD@github.com/packom/pca9956b-api
                    '''
                }
            }
        }
        stage('Auto-gen') {
            steps {
                sh '''
                    cd ~/builds && \
                    java -jar ~/openapi-generator/modules/openapi-generator-cli/target/openapi-generator-cli.jar generate --generate-alias-as-model -i ./pca9956b-api/api/openapi.yaml -g rust-server -o ./pca9956b-api
                    cd pca9956b-api && \
                    version = `awk '/^version / {print $3;}' Cargo.toml | sed 's/"//g'` && \
                    echo "# pca9956b-api

pca9956b-api is an HTTP RESTful API designed to control a PCA9956B IC bus.  This repo includes:
- An [API specification](https://github.com/packom/pca9956b-api/blob/master/api/openapi.yaml) in [OpenAPI format](https://github.com/OAI/OpenAPI-Specification/).
- Skeleton client and server implementations in [Rust](https://www.rust-lang.org/).

A fully-featured server implementation for Linux, in Rust, can be found at https://github.com/packom/pca9956b.

The text below was automatically generated by the openapi-generator.
" > /tmp/README.md && \
                    cat ./README.md >> /tmp/README.md && \
                    cp /tmp/README.md ./ && \
                    echo "[package]
name = \\"pca9956b-api\\"
version = \\"${version}\\"
authors = [\\"Piers Finlayson <piers@packom.net>\\"]
edition = \\"2018\\"
license = \\"GPL-3.0-or-later\\"
repository = \\"https://github.com/packom/pca9956b-api\\"
documentation = \\"https://github.com/packom/pca9956b-api\\"
homepage = \\"https://github.com/packom/pca9956b-api\\"
description = \\"HTTP RESTful API and skeleton server/client implement for I2C bus control\\"
readme = \\"README.md\\"
keywords = [\\"i2c\\",\\"bus\\",\\"openapi\\",\\"swagger\\",\\"http\\"]
categories = [\\"api-bindings\\",\\"hardware-support\\",\\"network-programming\\",\\"embedded\\",\\"web-programming\\"]
" > /tmp/Cargo.toml && \
                    tail -n +9 ./Cargo.toml >> /tmp/Cargo.toml && \
                    cp /tmp/Cargo.toml ./ && \
                    find examples -name *.rs -print0 | xargs -0 sed -i 's/openapi_client/pca9956b_api/'
                '''
            }
        }
        stage('Build') {
            steps {
                sh '''
                    cd ~/builds/pca9956b-api && \
                    cargo build
                '''
            }
        }
        stage('Test') {
            steps {
                sh '''
                    cd ~/builds/pca9956b-api && \
                    cargo test
                '''
            }
        }
        stage('Check in') {
            steps {
                sh '''
                    cd ~/builds/pca9956b-api && \
                    git config --global user.email "piers@packom.net" && \
                    git config --global user.name "Piers Finlayson" && \
                    git status && \
                    git diff && \
                    git diff -- . ':(exclude)README.md' > /tmp/diff && \
                    cat /tmp/diff
                    diff_size = `stat --printf="%s" /tmp/diff` && \
                    echo ${diff_size} && \
                    if [ ${diff_size} != 0 ] ; then git add -A && git commit -m "Checking in newly autogenerated version" && git push ; else echo "No changes to check in" ; fi
                '''
            }
        }
        stage('Publish') {
            steps {
                withCredentials([usernamePassword(credentialsId: 'crates.packom', usernameVariable: 'USERNAME', passwordVariable: 'PASSWORD')]) {
                    // Note yank failure is OK because there'll be nothing to yank if we have a new version
                    sh '''
                        cd ~/builds/pca9956b-api && \
                        if [ ${diff_size} != 0 ] ; then cargo yank --token $PASSWORD --version ${version} || true ; else echo "No changes to publish" ; fi && \
                        if [ ${diff_size} != 0 ] ; then cargo publish --token $PASSWORD ; else echo "No changes to publish" ; fi
                    '''
                }
            }
        }
    }
}
