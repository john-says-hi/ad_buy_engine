build-and-upload-campaign-server:
	make build-campaign-server && make upload-campaign-server

docker-volume-reset:
	ssh ad_buy_engine@72.14.190.165 'docker-compose down && docker volume rm campaign_server_storage && docker volume create --name=campaign_server_storage'

update-frontend:
	make build-secure-frontend && make docker-down && make upload-frontend && make docker-up

update-migration:
	make docker-down && make upload-migrations && make docker-up

update-server:
	make build-campaign-server && make docker-down && make upload-campaign-server && make docker-up

server-upload-all:
	make upload-static &&  scp ./Dockerfile ad_buy_engine@72.14.190.165:~/ && scp ./docker-compose.yml ad_buy_engine@72.14.190.165:~/ && scp -r ./migrations ad_buy_engine@72.14.190.165:~/ && scp -C ./target/release/campaign_server ad_buy_engine@72.14.190.165:~/bin/ && scp -C ./GeoLite2-ASN.mmdb ad_buy_engine@72.14.190.165:~/ && scp -C ./GeoLite2-City.mmdb ad_buy_engine@72.14.190.165:~/ && scp -C ./GeoLite2-Country.mmdb ad_buy_engine@72.14.190.165:~/ && scp ./regexes.yaml ad_buy_engine@72.14.190.165:~/ && scp .env ad_buy_engine@72.14.190.165:~/

upload-migrations:
	scp -r ./migrations ad_buy_engine@72.14.190.165:~/

update-docker-files:
	scp ./Dockerfile ad_buy_engine@72.14.190.165:~/ && scp ./docker-compose.yml ad_buy_engine@72.14.190.165:~/

docker-down:
	ssh ad_buy_engine@72.14.190.165 'docker-compose down'

docker-up:
	ssh ad_buy_engine@72.14.190.165 'docker-compose up -d'

fix-hashnames:
	cd dist/ && sed -i 's/index-.*.js/abe.js/g' index.html && sed -i 's/index-.*.wasm/abe.wasm/g' index.html && mv ./*.js ./abe.js && mv ./*.wasm ./abe.wasm &&  cd ..

rename-js-file:
	mv ./*.js ./abe.js

rename-wasm-file:
	mv ./*.wasm ./abe.wasm

start-server:
	sshpass -f "~/.abe.pw" ssh ad_buy_engine@72.14.190.165 ' ./campaign_server'

#stop-server:
	#curl -X GET "https://www.adbuyengine.com/stop" || true

check-frontend:
	cargo check -p frontend

check-server:
	cargo check -p campaign_server --features=backend

build-campaign-server:
	cargo build -p campaign_server --features=backend --release && cp target/release/campaign_server bin/

upload-static:
		scp -r ./static/ ad_buy_engine@72.14.190.165:~/

upload-frontend:
	scp ./static/main/secure/index.html ad_buy_engine@72.14.190.165:~/static/main/secure/ && scp ./static/main/secure/abe.js ad_buy_engine@72.14.190.165:~/static/main/secure/ && scp ./static/main/secure/abe.wasm ad_buy_engine@72.14.190.165:~/static/main/secure/ && scp ./static/main/secure/snippets/frontend-f18a95a0c5c4e16d/src/utils/javascript/js-scripts.js ad_buy_engine@72.14.190.165:~/static/main/secure/snippets/frontend-f18a95a0c5c4e16d/src/utils/javascript/js-scripts.js

upload-env:
	scp ./.env ad_buy_engine@72.14.190.165:~/

save:
	git add . && git commit -m "Auto Save" && git push

upload-campaign-server:
	scp -C ./target/release/campaign_server ad_buy_engine@72.14.190.165:~/bin/

build-secure-frontend:
	rm -rf static/main/secure/* || true && cd frontend && trunk clean && trunk build  --release --public-url secure && cd dist/ && sed -i 's/index-.*.js/abe.js/g' index.html && sed -i 's/index-.*.wasm/abe.wasm/g' index.html && mv ./*.js ./abe.js && mv ./*.wasm ./abe.wasm &&  cd .. && mv dist/* ../static/main/secure/ && trunk clean && cd ..


build-and-upload-tertiary:
	rm -rf static/main/public/tertiary/* || true  && cd tertiary_frontend/ && trunk clean && trunk build --release --public-url tertiary && cd .. && make tert-delete-files && make tert-copy-files && scp -r ./static/main/public/tertiary/* ad_buy_engine@72.14.190.165:~/static/main/public/tertiary/

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

