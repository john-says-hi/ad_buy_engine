#docker-save1:
#	docker save <my_image> | ssh -C user@my.remote.host.com docker load  && docker push valknutengine/ad-buy-engine:tagname
#docker-save:
#	docker save -o . ad_buy_engine &&  sudo docker login --username=valknutengine --email=johnwalton@protonmail.com
#sudo docker run --name postgres_alpine -e POSTGRES_PASSWORD=uiJWhNm7k5zAtn79qThEtQFB -d postgres
#docker inspect --format '{{.State.Pid}}' 385ef826cd14
#nsenter --target 3677 --mount --uts --ipc --net --pid
#docker exec -ti NAME_OF_CONTAINER psql -U YOUR_POSTGRES_USERNAME
#docker tag valknutengine/ad-buy-engine valknutengine/ad-buy-engine
#docker push docker-registry-username/docker-image-name
#docker commit -m "added Node.js" -a "sammy" d9b100f2f636 sammy/ubuntu-nodejs

#sudo docker run -d -p 5432:5432 e4784b8b1c35

#docker tag local-image:tagname new-repo:tagname
#docker push new-repo:tagname

#sudo certbot certonly --manual --preferred-challenges=dns --email johnwalton@protonmail.com --server https://acme-v02.api.letsencrypt.org/directory -d adbuyengine.com -d '*.adbuyengine.com'
#run-all:
#	make build-secure-frontend && make build-public-frontend && make build-n-run-master-server

#build-click-server:
#	cargo build -p click_server --release --features use-ua-parser

fix-hashnames:
	cd dist/ && sed -i 's/index-.*.js/abe.js/g' index.html && sed -i 's/index-.*.wasm/abe.wasm/g' index.html && mv ./*.js ./abe.js && mv ./*.wasm ./abe.wasm &&  cd ..

#sed-indexhtml:
#	sed -i 's/index-.*.js/abe.js/g' index.html && sed -i 's/index-.*.wasm/abe.wasm/g' index.html

rename-js-file:
	mv ./*.js ./abe.js

rename-wasm-file:
	mv ./*.wasm ./abe.wasm

start-server:
	sshpass -f "~/.abe.pw" ssh ad_buy_engine@72.14.190.165 ' ./campaign_server'

stop-server:
	curl -X GET "https://www.adbuyengine.com/stop" || true

check-frontend:
	cargo check -p frontend

check-server:
	cargo check -p campaign_server --features=backend

build-and-upload-server:
	make stop-server && make build-server && make upload-server

build-and-upload-frontend:
	make stop-server && make build-secure-frontend && make upload-frontend-lean

build-server:
	cargo build -p campaign_server --features=backend --release

upload-static:
		scp -r ./static/ ad_buy_engine@72.14.190.165:~/

upload-frontend-lean:
	scp ./static/main/secure/index.html ad_buy_engine@72.14.190.165:~/static/main/secure/ && scp ./static/main/secure/abe.js ad_buy_engine@72.14.190.165:~/static/main/secure/ && scp ./static/main/secure/abe.wasm ad_buy_engine@72.14.190.165:~/static/main/secure/ && scp ./static/main/secure/snippets/frontend-f18a95a0c5c4e16d/src/utils/javascript/js-scripts.js ad_buy_engine@72.14.190.165:~/static/main/secure/snippets/frontend-f18a95a0c5c4e16d/src/utils/javascript/js-scripts.js

upload-frontend-all:
	scp -r -C ./static/main/secure/* ad_buy_engine@72.14.190.165:~/static/main/secure/

upload-env:
	scp ./.env ad_buy_engine@72.14.190.165:~/

save:
	git add . && git commit -m "Auto Save" && git push -u origin master

upload-server:
	scp -C ./target/release/campaign_server ad_buy_engine@72.14.190.165:~/

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

