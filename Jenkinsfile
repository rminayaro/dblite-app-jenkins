pipeline {
    agent any
    environment {
        DOCKER_REGISTRY = "localhost:8082"           // URL de tu registro Docker (Nexus)
        DOCKER_IMAGE = "dblite_app"                  // Nombre de la imagen
        DOCKER_TAG = "latest"                        // Etiqueta de la imagen Docker
        SERVER_USER = "root"                         // Usuario en el servidor remoto
        SERVER_IP = "64.23.161.84"                  // Dirección IP del servidor
        SERVER_PASSWORD = "Ramon2Minaya"            // Contraseña del servidor
        GITHUB_CREDENTIALS = "github-credentials"   // Credenciales de GitHub
        GITHUB_REPO = "https://github.com/rminayaro/dblite-app-jenkins.git" // URL del repositorio GitHub
        NEXUS_USER = "admin"                        // Usuario Nexus
        NEXUS_PASSWORD = "123456"                   // Contraseña Nexus
        PLINK_PATH = "\"C:\\Program Files\\PuTTY\\plink.exe\"" // Ruta correcta de plink.exe
    }

    stages {
        stage('Checkout') {
            steps {
                echo "📥 Clonando código fuente desde GitHub..."
                git branch: env.BRANCH_NAME, credentialsId: GITHUB_CREDENTIALS, url: GITHUB_REPO
            }
        }

        stage('Check Branch') {
            steps {
                script {
                    def branchName = env.BRANCH_NAME
                    if (branchName == 'main') {
                        error("❌ No se puede desplegar automáticamente en 'main'. Solo mediante Pull Request.")
                    } else {
                        echo "✅ Ejecutando en la rama '${branchName}', se permite el despliegue."
                    }
                }
            }
        }

        stage('Build Docker Image') {
            steps {
                echo "🔨 Construyendo imagen Docker..."
                bat "docker build -t ${DOCKER_REGISTRY}/${DOCKER_IMAGE}:${DOCKER_TAG} ."
            }
        }

        stage('Login to Nexus') {
            steps {
                echo "🔑 Iniciando sesión en Nexus..."
                bat "docker login -u ${NEXUS_USER} -p '${NEXUS_PASSWORD}' ${DOCKER_REGISTRY}"
            }
        }

        stage('Push to Nexus') {
            steps {
                echo "📤 Subiendo imagen a Nexus..."
                bat "docker push ${DOCKER_REGISTRY}/${DOCKER_IMAGE}:${DOCKER_TAG}"
            }
        }

        stage('Merge Check') {
            when {
                branch 'main'
            }
            steps {
                echo "🔍 Verificando si el código en 'main' está actualizado antes del despliegue..."
                script {
                    def gitStatus = bat(script: 'git status', returnStdout: true).trim()
                    if (gitStatus.contains('Your branch is behind')) {
                        error("❌ La rama 'main' no está actualizada. Debes hacer un `git pull` antes de desplegar.")
                    } else {
                        echo "✅ La rama 'main' está actualizada. Listo para desplegar."
                    }
                }
            }
        }

        stage('Deploy to Server') {
            when {
                branch 'main'
            }
            steps {
                echo "🚀 Desplegando aplicación en el servidor..."
                script {
                    bat """
                        ${PLINK_PATH} -batch -ssh ${SERVER_USER}@${SERVER_IP} -pw ${SERVER_PASSWORD} -o StrictHostKeyChecking=no ^
                        "docker pull ${DOCKER_REGISTRY}/${DOCKER_IMAGE}:${DOCKER_TAG} && ^
                        docker stop ${DOCKER_IMAGE} || true && ^
                        docker rm -f ${DOCKER_IMAGE} || true && ^
                        docker run -d --restart unless-stopped --name ${DOCKER_IMAGE} -p 3030:3030 ${DOCKER_REGISTRY}/${DOCKER_IMAGE}:${DOCKER_TAG}"
                    """
                }
            }
        }
    }

    post {
        success {
            echo "🎉 Despliegue exitoso de la aplicación!"
        }
        failure {
            echo "🚨 ERROR en el despliegue!"
        }
    }
}
