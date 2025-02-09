pipeline {
    agent any
    environment {
        DOCKER_REGISTRY = "localhost:8082"
        DOCKER_IMAGE = "dblite_app"
        DOCKER_TAG = "latest"
        SERVER_USER = "root"
        SERVER_IP = "64.23.161.84"
        SSH_CREDENTIALS = "ssh-server-credentials"
        GITHUB_CREDENTIALS = "github-credentials"
        GITHUB_REPO = "https://github.com/rminayaro/dblite-app-jenkins.git"
        NEXUS_USER = "admin"
        NEXUS_PASSWORD = "123456"
        SERVER_PASSWORD = "Ramon2Minaya" // Agrega la contraseña del servidor aquí
    }
    stages {
        stage('Checkout') {
            steps {
                echo "📥 Clonando código fuente desde GitHub..."
                git branch: 'develop', credentialsId: GITHUB_CREDENTIALS, url: GITHUB_REPO
            }
        }
        stage('Build Docker Image') {
            steps {
                echo "🔨 Construyendo imagen Docker..."
                sh "docker build -t ${DOCKER_REGISTRY}/${DOCKER_IMAGE}:${DOCKER_TAG} ."
            }
        }
        stage('Login to Nexus') {
            steps {
                echo "🔑 Iniciando sesión en Nexus..."
                sh "docker login -u ${NEXUS_USER} -p '${NEXUS_PASSWORD}' ${DOCKER_REGISTRY}"
            }
        }
        stage('Push to Nexus') {
            steps {
                echo "📤 Subiendo imagen a Nexus..."
                sh "docker push ${DOCKER_REGISTRY}/${DOCKER_IMAGE}:${DOCKER_TAG}"
            }
        }
        stage('Deploy to Server') {
            steps {
                echo "🚀 Desplegando aplicación en el servidor..."
                script {
                    // Ejecutar comandos SSH con autenticación por contraseña utilizando sshpass
                    sh """
                    sshpass -p '${SERVER_PASSWORD}' ssh -o StrictHostKeyChecking=no ${SERVER_USER}@${SERVER_IP} << 'ENDSSH'
                    docker pull ${DOCKER_REGISTRY}/${DOCKER_IMAGE}:${DOCKER_TAG}
                    docker stop ${DOCKER_IMAGE} || true
                    docker rm -f ${DOCKER_IMAGE} || true
                    docker run -d --restart unless-stopped --name ${DOCKER_IMAGE} -p 3030:3030 ${DOCKER_REGISTRY}/${DOCKER_IMAGE}:${DOCKER_TAG}
                    exit
                    ENDSSH
                    """
                }
            }
        }
    }
    post {
        success {
            echo "🎉 Despliegue exitoso de Rust API!"
        }
        failure {
            echo "🚨 ERROR en el despliegue!"
        }
    }
}
