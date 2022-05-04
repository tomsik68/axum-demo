k8s_yaml(helm('helm'))
docker_build("axum-demo", ".")

k8s_resource(
  workload='axum-demo',
  port_forwards=8080
)
