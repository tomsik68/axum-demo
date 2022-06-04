.PHONY: new-cluster del-cluster

CLUSTER ?= axum-demo

new-cluster:
	k3d registry create registry.local --port 47616
	k3d cluster create $(CLUSTER) --registry-use k3d-registry.local:47616

del-cluster:
	k3d cluster delete $(CLUSTER)

dev:
	tilt up
