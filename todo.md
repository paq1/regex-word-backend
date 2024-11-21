# todo

- [ ] Creation d'un regexword
  - [ ] mot ne doit pas exister dans le référentiel regexword, (erreur 400) 
  - [x] si mot < 3 caractere (erreur 400)
  - [ ] mot ne doit pas contenir d'accents ou caractere autre que [a-z]{3,} (erreur 400)
  - [x] généré une regex pour le mot
    - [x] si le nombre de regex > 3 alors merge les derniers elements
- [x] Selection d'un regexword
  - [x] +1 nb_selected et last_date_selected = today
  - [x] il ne doit pas y avoir de mot selectionné pour aujourd'hui
- [ ] Fetch mot du jour
  - [ ] Si pas de mot pour aujourd'hui en selectionner 1
  - [ ] Masquer le mot du jour pour le moment
- [ ] Verifier le mot du jour
  - [ ] Si pas de mot du jour, lever une erreur (erreur 500)
- [ ] Déployer le backend en prod:
  - [ ] configurer le sous domain
  - [ ] mettre la config nginx
  - [ ] généré les certificat ssl
