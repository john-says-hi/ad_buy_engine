DEPLOY_TARGET ?= deploy_user@deploy_host
DEPLOY_HOME ?= /home/app

build-and-upload-campaign-server:
	make build-campaign-server && make upload-campaign-server

docker-volume-reset:
	ssh $(DEPLOY_TARGET) 'docker-compose down && docker volume rm campaign_server_storage && docker volume create --name=campaign_server_storage'

update-frontend:
	make build-secure-frontend && make docker-down && make upload-frontend && make docker-up && firefox -new-tab "https://adbuyengine.com/secure/"

update-migration:
	make docker-down && make upload-migrations && make docker-up

update-server:
	make build-campaign-server && make docker-down && make upload-campaign-server && make docker-up-it

server-upload-all:
	make upload-static && scp ./Dockerfile $(DEPLOY_TARGET):~/ && scp ./docker-compose.yml $(DEPLOY_TARGET):~/ && scp -r ./migrations $(DEPLOY_TARGET):~/ && scp -C ./target/release/campaign_server $(DEPLOY_TARGET):~/bin/ && scp -C ./GeoLite2-ASN.mmdb $(DEPLOY_TARGET):~/ && scp -C ./GeoLite2-City.mmdb $(DEPLOY_TARGET):~/ && scp -C ./GeoLite2-Country.mmdb $(DEPLOY_TARGET):~/ && scp ./regexes.yaml $(DEPLOY_TARGET):~/

upload-migrations:
	scp -r ./migrations $(DEPLOY_TARGET):~/

update-docker-files:
	scp ./Dockerfile $(DEPLOY_TARGET):~/ && scp ./docker-compose.yml $(DEPLOY_TARGET):~/

update-ovalhalla-css:
	scp ./static/main/public/assets/css/ovalhalla.css $(DEPLOY_TARGET):$(DEPLOY_HOME)/static/main/public/assets/css/ovalhalla.css

docker-down:
	ssh $(DEPLOY_TARGET) 'docker-compose down'

docker-up:
	ssh $(DEPLOY_TARGET) 'docker-compose up -d'

docker-up-it:
	ssh $(DEPLOY_TARGET) 'docker-compose up'

fix-hashnames:
	cd dist/ && sed -i 's/index-.*.js/abe.js/g' index.html && sed -i 's/index-.*.wasm/abe.wasm/g' index.html && mv ./*.js ./abe.js && mv ./*.wasm ./abe.wasm &&  cd ..

rename-js-file:
	mv ./*.js ./abe.js

rename-wasm-file:
	mv ./*.wasm ./abe.wasm

start-server:
	ssh $(DEPLOY_TARGET) './campaign_server'

#stop-server:
	#curl -X GET "https://www.adbuyengine.com/stop" || true

check-frontend:
	cargo check -p frontend

check-server:
	cargo check -p campaign_server --features=backend

build-campaign-server:
	cargo build -p campaign_server --features=backend --release && cp target/release/campaign_server bin/

upload-static:
		scp -r ./static/ $(DEPLOY_TARGET):~/

upload-frontend:
	scp ./static/main/secure/index.html $(DEPLOY_TARGET):~/static/main/secure/ && scp ./static/main/secure/abe.js $(DEPLOY_TARGET):~/static/main/secure/ && scp -C ./static/main/secure/abe.wasm $(DEPLOY_TARGET):~/static/main/secure/ && scp ./static/main/secure/snippets/frontend-f18a95a0c5c4e16d/src/utils/javascript/js-scripts.js $(DEPLOY_TARGET):~/static/main/secure/snippets/frontend-f18a95a0c5c4e16d/src/utils/javascript/js-scripts.js

upload-env:
	@echo "Refusing to upload .env from the public Makefile. Copy secrets through a secure deployment channel."

save:
	git add . && git commit -m "Auto Save" && git push

upload-campaign-server:
	scp -C ./target/release/campaign_server $(DEPLOY_TARGET):~/bin/

build-secure-frontend:
	rm -rf static/main/secure/* || true && cd frontend && trunk clean && trunk build  --release --public-url secure && cd dist/ && sed -i 's/index-.*.js/abe.js/g' index.html && sed -i 's/index-.*.wasm/abe.wasm/g' index.html && mv ./*.js ./abe.js && mv ./*.wasm ./abe.wasm &&  cd .. && mv dist/* ../static/main/secure/ && trunk clean && cd ..


build-and-upload-tertiary:
	rm -rf static/main/public/tertiary/* || true  && cd tertiary_frontend/ && trunk clean && trunk build --release --public-url tertiary && cd .. && make tert-delete-files && make tert-copy-files && scp -r ./static/main/public/tertiary/* $(DEPLOY_TARGET):~/static/main/public/tertiary/

tert-copy-files:
	cd tertiary_frontend/ && cd dist/ && sed -i 's/index-.*.js/p_abe.js/g' index.html && sed -i 's/index-.*.wasm/p_abe.wasm/g' index.html && mv ./*.js ./p_abe.js && mv ./*.wasm ./p_abe.wasm && cd .. && mv dist/* ../static/main/public/tertiary/ && trunk clean && cd ..

tert-delete-files:
	make tert-delete-index && make tert-delete-snippets && make tert-delete-js && make tert-delete-wasm

tert-delete-index:
	rm -f static/main/public/tertiary/index.html

tert-delete-snippets:
	rm -rf static/main/public/tertiary/snippets

tert-delete-js:
	rm -f static/main/public/tertiary/index-*.js

tert-delete-wasm:
	rm -f static/main/public/tertiary/index-*.wasm
