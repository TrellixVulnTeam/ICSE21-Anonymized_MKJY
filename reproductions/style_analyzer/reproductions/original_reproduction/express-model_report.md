# Model report for file:///tmp/top-repos-quality-repos-e3x98zh0/express HEAD b4eb1f59d39d801d7365c86b04500f16faeb0b1c

### Dump

```json
{'created_at': '2021-08-13 16:36:05',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.4.0-80-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.9 (default, Jan 26 2021, 15:33:00) [GCC 8.4.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '17.1 kB',
 'tags': [],
 'uuid': '370a3543-ae5f-4ffb-9b03-7b1357144aa3',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-e3x98zh0/express b4eb1f59d39d801d7365c86b04500f16faeb0b1c

# javascript
42 rules, avg.len. 8.7
## train
PPCR: 0.936035
### report
macro
{'f1-score': 0.9407837000611073,
 'precision': 0.9567170931743278,
 'recall': 0.9268900324182361,
 'support': 67256}
micro
{'f1-score': 0.9693410253360295,
 'precision': 0.9693410253360295,
 'recall': 0.9693410253360295,
 'support': 67256}
weighted
{'f1-score': 0.9688079143206172,
 'precision': 0.9689240782470719,
 'recall': 0.9693410253360295,
 'support': 67256}
### report_full
macro
{'f1-score': 0.8992554446010051,
 'precision': 0.9567170931743278,
 'recall': 0.854828117609037,
 'support': 71852}
micro
{'f1-score': 0.9373148920263391,
 'precision': 0.9693410253360295,
 'recall': 0.907337304459166,
 'support': 71852}
weighted
{'f1-score': 0.9352653488861141,
 'precision': 0.9680478345031822,
 'recall': 0.907337304459166,
 'support': 71852}
## test
PPCR: 0.929035
### report
macro
{'f1-score': 0.9156585989827993,
 'precision': 0.9446982027508062,
 'recall': 0.8924085666440117,
 'support': 14453}
micro
{'f1-score': 0.9575866602089531,
 'precision': 0.9575866602089531,
 'recall': 0.9575866602089531,
 'support': 14453}
weighted
{'f1-score': 0.9560831645065708,
 'precision': 0.9565873113187495,
 'recall': 0.9575866602089531,
 'support': 14453}
### report_full
macro
{'f1-score': 0.8733555562286369,
 'precision': 0.9446982027508062,
 'recall': 0.8228878707973962,
 'support': 15557}
micro
{'f1-score': 0.9223592135954681,
 'precision': 0.9575866602089531,
 'recall': 0.8896316770585588,
 'support': 15557}
weighted
{'f1-score': 0.9186367685228695,
 'precision': 0.9552096563572057,
 'recall': 0.8896316770585588,
 'support': 15557}
```

## javascript
### Summary
42 rules, avg.len. 8.7

| | |
|-|-|
|Min support|96|
|Max support|22584|
|Min confidence|0.828125|
|Max confidence|0.9998694658279419|

### Configuration

```json
{'feature_extractor': {'cutoff_label_support': 80,
                       'debug_parsing': False,
                       'label_composites': '<cut>',
                       'left_features': ['length',
                                         'diff_offset',
                                         'diff_col',
                                         'diff_line',
                                         'internal_type',
                                         'label',
                                         'reserved',
                                         'roles'],
                       'left_siblings_window': 5,
                       'no_labels_on_right': True,
                       'node_features': ['start_line', 'start_col'],
                       'parent_features': ['internal_type', 'roles'],
                       'parents_depth': 2,
                       'return_sibling_indices': False,
                       'right_features': ['length', 'internal_type', 'reserved', 'roles'],
                       'right_siblings_window': 5,
                       'select_features_number': 500,
                       'selected_features': '<cut>'},
 'line_length_limit': 500,
 'lines_ratio_train_trigger': 0.2,
 'lower_bound_instances': 500,
 'optimizer': {'base_model_name_categories': ['sklearn.ensemble.RandomForestClassifier',
                                              'sklearn.tree.DecisionTreeClassifier'],
               'cv': 3,
               'max_depth_categories': [None, 5, 10],
               'max_features_categories': [None, 'auto'],
               'min_samples_leaf_max': 130,
               'min_samples_leaf_min': 90,
               'min_samples_split_max': 240,
               'min_samples_split_min': 180,
               'n_iter': 50,
               'n_jobs': -1},
 'overall_size_limit': 5242880,
 'random_state': 42,
 'test_dataset_ratio': 0.2,
 'trainable_rules': {'attribute_similarity_threshold': 0.98,
                     'base_model_name': 'sklearn.tree.DecisionTreeClassifier',
                     'confidence_threshold': 0.8,
                     'min_samples_leaf': 90,
                     'min_samples_split': 180,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.roles in {STRING}<br>⇒ y = '<br>Confidence: 1.000. Support: 3831.` |
| 2 | `  -1.reserved = ,<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles in {STRING}<br>⇒ y = ␣<br>Confidence: 0.965. Support: 592.` |
| 3 | `  -1.reserved = :<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles in {STRING}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 222.` |
| 4 | `  -1.reserved not in {:}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label in {<space>}<br>	∧ +1.roles in {STRING}<br>⇒ y = ␣<br>Confidence: 0.895. Support: 204.` |
| 5 | `  -1.reserved not in {,, :}<br>	∧ -2.label not in {<space>}<br>	∧ +1.roles in {STRING}<br>⇒ y = '<br>Confidence: 0.988. Support: 3916.` |
| 6 | `  -1.reserved = ,<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {STRING}<br>⇒ y = ␣<br>Confidence: 0.981. Support: 3028.` |
| 7 | `  -1.reserved not in {,}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_offset ≥ 3<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {STRING}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1282.` |
| 8 | `  -1.reserved = var<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_offset ≥ 3<br>	∧ +1.roles not in {STRING}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 883.` |
| 9 | `  -1.reserved not in {var}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -4.label in {<space>}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {STRING}<br>⇒ y = ␣<br>Confidence: 0.868. Support: 140.` |
| 10 | `  -1.reserved not in {,, var}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -4.label not in {<space>}<br>	∧ -4.roles not in {IDENTIFIER}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {STRING}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.986. Support: 257.` |
| 11 | `  -1.reserved not in {,, var}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -4.roles in {CALLEE}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.roles not in {STRING}<br>	∧ +4.internal_type = StringLiteral<br>⇒ y = ⏎<br>Confidence: 0.954. Support: 441.` |
| 12 | `  -1.reserved not in {,, var}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -4.roles in {CALLEE}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.roles not in {STRING}<br>	∧ +4.internal_type not in {StringLiteral}<br>⇒ y = ∅<br>Confidence: 0.920. Support: 143.` |
| 13 | `  -1.reserved not in {var}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -4.roles not in {CALLEE}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.929. Support: 404.` |
| 14 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {,, var}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -4.roles not in {CALLEE}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.835. Support: 325.` |
| 15 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = if<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -4.roles not in {CALLEE}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {STRING}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 201.` |
| 16 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = :<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -4.roles not in {CALLEE}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {STRING}<br>⇒ y = ␣<br>Confidence: 0.965. Support: 186.` |
| 17 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = return<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -4.roles not in {CALLEE}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {STRING}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 174.` |
| 18 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -4.label in {<space>}<br>	∧ -4.roles not in {CALLEE}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {STRING}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 184.` |
| 19 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -4.label not in {<space>}<br>	∧ -4.roles not in {CALLEE}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.929. Support: 119.` |
| 20 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {,, :, var}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -4.label not in {<space>}<br>	∧ -4.roles not in {CALLEE}<br>	∧ -4.length ≥ 3<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR, STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.922. Support: 457.` |
| 21 | `  •••start_col ≤ 33<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {,, :, var}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -4.label not in {<space>}<br>	∧ -4.roles not in {CALLEE}<br>	∧ -4.length ≤ 2<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR, STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.892. Support: 134.` |
| 22 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {,, ;, var}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.internal_type = CommentBlock<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎⏎<br>Confidence: 0.833. Support: 129.` |
| 23 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = new<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -4.roles not in {CALLEE}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {STRING}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 110.` |
| 24 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {,, ;, function, new, var}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -4.roles not in {CALLEE}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {IF} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.876. Support: 350.` |
| 25 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {,, ;, function, var}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.label in {<-space>}<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IF, OPERATOR}<br>⇒ y = ⏎⏎<br>Confidence: 0.892. Support: 106.` |
| 26 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {,, ;, function, new, var}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.label in {<-space>}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -4.roles not in {CALLEE}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IF, OPERATOR}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 1354.` |
| 27 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {,, :, ;, function, var}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.roles not in {LITERAL}<br>	∧ -4.roles not in {CALLEE}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {IF, OPERATOR}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 22584.` |
| 28 | `  -1.reserved = {<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_offset ≤ 2<br>	∧ +1.roles not in {STRING}<br>	∧ +3.roles in {VALUE}<br>⇒ y = ␣<br>Confidence: 0.907. Support: 189.` |
| 29 | `  -1.reserved = {<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_offset ≤ 2<br>	∧ +1.roles not in {STRING}<br>	∧ +3.roles not in {VALUE}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.975. Support: 1766.` |
| 30 | `  -1.reserved not in {,, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_offset ≤ 2<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {STRING}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.960. Support: 1479.` |
| 31 | `  -1.reserved = =<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_offset ≤ 2<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {STRING}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 1291.` |
| 32 | `  -1.reserved = ;<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_offset ≤ 2<br>	∧ -4.diff_col ≥ 9<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {STRING}<br>	∧ +3.length ≥ 2<br>⇒ y = ⏎⏎<br>Confidence: 0.905. Support: 850.` |
| 33 | `  •••start_col ≤ 11<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_offset ≤ 2<br>	∧ -4.diff_col ≤ 8<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {STRING}<br>	∧ +3.length ≥ 2<br>⇒ y = ⏎⏎<br>Confidence: 0.969. Support: 114.` |
| 34 | `  -1.reserved = ;<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_offset ≤ 2<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {STRING}<br>	∧ +3.length ≤ 1<br>⇒ y = ⏎<br>Confidence: 0.850. Support: 409.` |
| 35 | `  -1.reserved not in {,, ;, =, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_offset ≤ 2<br>	∧ +1.reserved = .<br>	∧ +1.roles not in {STRING}<br>	∧ +4.roles in {CALL}<br>⇒ y = ⏎<br>Confidence: 0.968. Support: 663.` |
| 36 | `  -1.reserved not in {,, ;, =, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_offset ≤ 2<br>	∧ +1.reserved not in {., }}<br>	∧ +1.roles in {CALLEE} and not in {STRING}<br>⇒ y = ⏎⏎<br>Confidence: 0.948. Support: 472.` |
| 37 | `  -1.reserved not in {;, =, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_offset ≤ 2<br>	∧ +1.reserved not in {., }}<br>	∧ +1.roles not in {CALLEE, STRING}<br>	∧ ^1.internal_type = BinaryExpression<br>⇒ y = ␣<br>Confidence: 0.911. Support: 207.` |
| 38 | `  -1.reserved not in {,, ;, =, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_offset ≤ 2<br>	∧ +1.reserved not in {., }}<br>	∧ +1.roles not in {CALLEE, STRING}<br>	∧ +3.reserved = =<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {STATEMENT}<br>⇒ y = ⏎<br>Confidence: 0.828. Support: 96.` |
| 39 | `  -1.reserved not in {,, ;, =, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_offset ≤ 2<br>	∧ -5.diff_col ≥ 1<br>	∧ +1.reserved not in {., }}<br>	∧ +1.roles not in {CALLEE, STRING}<br>	∧ +2.reserved = .<br>	∧ +3.reserved not in {=}<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^2.roles not in {ARGUMENT}<br>⇒ y = ∅<br>Confidence: 0.872. Support: 183.` |
| 40 | `  -1.reserved not in {,, ;, =, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_offset ≤ 2<br>	∧ -3.reserved = function<br>	∧ -5.diff_col ≥ 1<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {CALLEE, STRING}<br>	∧ +3.reserved not in {=}<br>	∧ ^1.internal_type not in {BinaryExpression}<br>⇒ y = ∅<br>Confidence: 0.963. Support: 362.` |
| 41 | `  -1.reserved not in {;, =, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_offset ≤ 2<br>	∧ -3.reserved not in {function}<br>	∧ -5.diff_col ≥ 1<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {CALLEE, STRING}<br>	∧ +3.reserved not in {=}<br>⇒ y = ␣<br>Confidence: 0.894. Support: 127.` |
| 42 | `  -1.reserved not in {,, ;, =, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_offset ≤ 2<br>	∧ -5.diff_col ≥ 1<br>	∧ +1.reserved not in {., {, }}<br>	∧ +1.roles not in {CALLEE, STRING}<br>	∧ +2.reserved not in {.}<br>	∧ +3.reserved not in {=}<br>	∧ ^1.internal_type not in {BinaryExpression}<br>⇒ y = ∅<br>Confidence: 0.951. Support: 3648.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.69047619047619, "max_conf": 0.9998694658279419, "max_support": 22584, "min_conf": 0.828125, "min_support": 96, "num_rules": 42}}
```
</details>
