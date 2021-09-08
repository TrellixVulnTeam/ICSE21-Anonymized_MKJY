# Model report for file:///tmp/top-repos-quality-repos-q9ykj62p/storybook HEAD b28217f887af533a17cb1498887d6b4bd41bd643

### Dump

```json
{'created_at': '2021-08-13 17:16:45',
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
 'size': '20.6 kB',
 'tags': [],
 'uuid': 'b1699639-b551-4be9-8cdb-2059979f0b5d',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-q9ykj62p/storybook b28217f887af533a17cb1498887d6b4bd41bd643

# javascript
78 rules, avg.len. 9.8
## train
PPCR: 0.936332
### report
macro
{'f1-score': 0.6921807130458807,
 'precision': 0.7664453166249374,
 'recall': 0.6572871905546245,
 'support': 146610}
micro
{'f1-score': 0.9552008730645931,
 'precision': 0.9552008730645931,
 'recall': 0.9552008730645931,
 'support': 146610}
weighted
{'f1-score': 0.9534511973780014,
 'precision': 0.9543694431905657,
 'recall': 0.9552008730645931,
 'support': 146610}
### report_full
macro
{'f1-score': 0.6243874115536464,
 'precision': 0.7664453166249374,
 'recall': 0.584744438710213,
 'support': 156579}
micro
{'f1-score': 0.9237934093915017,
 'precision': 0.9552008730645931,
 'recall': 0.8943855817191322,
 'support': 156579}
weighted
{'f1-score': 0.9088792581930754,
 'precision': 0.9535801886302716,
 'recall': 0.8943855817191322,
 'support': 156579}
## test
PPCR: 0.938787
### report
macro
{'f1-score': 0.7043084719921247,
 'precision': 0.7675497564644305,
 'recall': 0.6707255366441703,
 'support': 33970}
micro
{'f1-score': 0.9574624668825434,
 'precision': 0.9574624668825434,
 'recall': 0.9574624668825434,
 'support': 33970}
weighted
{'f1-score': 0.9560852745410817,
 'precision': 0.9565388274502116,
 'recall': 0.9574624668825434,
 'support': 33970}
### report_full
macro
{'f1-score': 0.6292842794617992,
 'precision': 0.7675497564644305,
 'recall': 0.5915185518790788,
 'support': 36185}
micro
{'f1-score': 0.9272325564820754,
 'precision': 0.9574624668825434,
 'recall': 0.898853115932016,
 'support': 36185}
weighted
{'f1-score': 0.912566543562087,
 'precision': 0.9555501598852928,
 'recall': 0.898853115932016,
 'support': 36185}
```

## javascript
### Summary
78 rules, avg.len. 9.8

| | |
|-|-|
|Min support|94|
|Max support|42449|
|Min confidence|0.8056994676589966|
|Max confidence|0.9996851682662964|

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
| 1 | `  -1.internal_type = StringLiteral<br>	∧ -2.reserved = =<br>⇒ y = "<br>Confidence: 0.996. Support: 345.` |
| 2 | `  -1.internal_type = StringLiteral<br>	∧ -2.reserved not in {=}<br>⇒ y = '<br>Confidence: 0.996. Support: 6180.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.999. Support: 3825.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.954. Support: 1704.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<newline>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.981. Support: 395.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.internal_type = JSXIdentifier<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.995. Support: 320.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = [<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.919. Support: 314.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +2.reserved = ]<br>	∧ ^1.roles in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 97.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.972. Support: 3777.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.984. Support: 1644.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = import<br>⇒ y = ⏎<br>Confidence: 0.855. Support: 1061.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {import, }}<br>	∧ +1.length = 0<br>	∧ ^1.roles not in {MODULE}<br>⇒ y = ⏎<br>Confidence: 0.893. Support: 530.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {VALUE}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.868. Support: 261.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {VALUE}<br>	∧ ^1.roles in {DECLARATION, FUNCTION}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.963. Support: 905.` |
| 15 | `  •••start_col ≥ 26<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {VALUE}<br>	∧ ^1.roles in {DECLARATION} and not in {FUNCTION}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.927. Support: 103.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.roles in {DECLARATION, FUNCTION}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 217.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^1.roles in {DECLARATION, FUNCTION}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 216.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +2.roles not in {INCOMPLETE}<br>	∧ ^1.roles in {DECLARATION, FUNCTION}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 1713.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {DECLARATION, FUNCTION}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 564.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^1.roles in {DECLARATION, FUNCTION}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 192.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.roles in {DECLARATION, FUNCTION}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 650.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {INCOMPLETE}<br>	∧ ^1.roles in {DECLARATION, FUNCTION}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 94.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {INCOMPLETE}<br>	∧ ^1.roles in {DECLARATION, FUNCTION}<br>⇒ y = ␣<br>Confidence: 0.950. Support: 2437.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles in {DECLARATION} and not in {FUNCTION}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 461.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -3.diff_col ≥ 5<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {,}<br>	∧ ^1.roles in {DECLARATION} and not in {FUNCTION}<br>⇒ y = ␣<br>Confidence: 0.972. Support: 6540.` |
| 26 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -3.diff_col ≤ 4<br>	∧ -3.reserved = =<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {,}<br>	∧ ^1.roles in {DECLARATION} and not in {FUNCTION}<br>⇒ y = ∅<br>Confidence: 0.872. Support: 215.` |
| 27 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -2.diff_offset ≥ 4<br>	∧ -3.diff_col ≤ 4<br>	∧ -3.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {,}<br>	∧ +2.reserved = ,<br>	∧ ^1.roles in {DECLARATION} and not in {FUNCTION}<br>⇒ y = ⏎<br>Confidence: 0.995. Support: 106.` |
| 28 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -2.diff_offset ≤ 3<br>	∧ -3.diff_col ≤ 4<br>	∧ -3.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {,}<br>	∧ ^1.roles in {DECLARATION} and not in {FUNCTION}<br>⇒ y = ␣<br>Confidence: 0.842. Support: 547.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.984. Support: 1364.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {KEY}<br>	∧ ^1.roles in {LITERAL} and not in {DECLARATION}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.921. Support: 1973.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles in {LITERAL} and not in {DECLARATION}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.851. Support: 198.` |
| 32 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -2.roles in {EXPRESSION}<br>	∧ -5.diff_line = 0<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles in {LITERAL} and not in {DECLARATION}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.806. Support: 193.` |
| 33 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -2.length ≥ 2<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {DECLARATION, LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.991. Support: 162.` |
| 34 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -5.diff_line = 0<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {DECLARATION, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.906. Support: 2122.` |
| 35 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.989. Support: 2891.` |
| 36 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {OPERATOR} and not in {DECLARATION}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.935. Support: 192.` |
| 37 | `  •••start_col ≥ 12<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.992. Support: 1189.` |
| 38 | `  •••start_col ≥ 12<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -2.label in {<space>}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.964. Support: 820.` |
| 39 | `  •••start_col ≥ 12<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -2.label not in {<space>}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 232.` |
| 40 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = const<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1588.` |
| 41 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.943. Support: 1106.` |
| 42 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved = }<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.984. Support: 532.` |
| 43 | `  •••start_col ≥ 11<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -3.diff_col ≥ 8<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {}}<br>	∧ +4.reserved = ,<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE, OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.875. Support: 108.` |
| 44 | `  •••start_col ≤ 34<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -3.diff_col ≤ 7<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved = :<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE, OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.851. Support: 575.` |
| 45 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -3.diff_col ≤ 7<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {}}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE, OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.984. Support: 1016.` |
| 46 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = import<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1399.` |
| 47 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, import, {}<br>	∧ -2.reserved = =<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.974. Support: 866.` |
| 48 | `  -1.internal_type = JSXIdentifier<br>	∧ -1.reserved not in {,, ;, import, {}<br>	∧ +1.internal_type = JSXIdentifier<br>	∧ +1.length ≤ 9<br>	∧ ^1.internal_type = JSXOpeningElement<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.890. Support: 497.` |
| 49 | `  •••start_col ≥ 43<br>	∧ -1.internal_type not in {JSXIdentifier}<br>	∧ -1.reserved not in {,, ;, import, {}<br>	∧ +1.internal_type = JSXIdentifier<br>	∧ +1.length ≤ 9<br>	∧ ^1.internal_type = JSXOpeningElement<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.862. Support: 149.` |
| 50 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, import, {}<br>	∧ -2.reserved not in {=}<br>	∧ +1.internal_type not in {JSXIdentifier, StringLiteral}<br>	∧ ^1.internal_type = JSXOpeningElement<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.899. Support: 569.` |
| 51 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = export<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 510.` |
| 52 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = return<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 510.` |
| 53 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, import, return, {}<br>	∧ -2.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved = ><br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 1003.` |
| 54 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, import, return, {}<br>	∧ -2.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved = ><br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 98.` |
| 55 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, return, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved = ><br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.953. Support: 1107.` |
| 56 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {,, :, ;, const, return, {}<br>	∧ -2.reserved not in {=}<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.862. Support: 496.` |
| 57 | `  -1.diff_col ≥ 2<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {NAME}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 373.` |
| 58 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, export, {}<br>	∧ -2.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {NAME}<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ⏎⏎<br>Confidence: 0.996. Support: 130.` |
| 59 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, import, {}<br>	∧ -2.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +2.reserved not in {>}<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.901. Support: 498.` |
| 60 | `  •••start_col ≥ 17<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, import, {}<br>	∧ -2.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +2.reserved not in {>}<br>	∧ +3.reserved = ><br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 124.` |
| 61 | `  •••start_col ≥ 17<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +3.reserved not in {>}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.870. Support: 652.` |
| 62 | `  •••start_col ≤ 16<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, export, {}<br>	∧ -2.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE, OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.948. Support: 202.` |
| 63 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 311.` |
| 64 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ ^1.roles in {IF} and not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.971. Support: 368.` |
| 65 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type = ConditionalExpression<br>	∧ ^1.roles in {IF} and not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.877. Support: 240.` |
| 66 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, import, {}<br>	∧ -2.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +2.reserved not in {>}<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles in {IF} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.894. Support: 1475.` |
| 67 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -3.reserved = export<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {DECLARATION, IF}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 251.` |
| 68 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if, import, {}<br>	∧ -2.label in {<-space>}<br>	∧ -2.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles not in {DECLARATION, IF, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 2849.` |
| 69 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if, import, {}<br>	∧ -2.label not in {<-space>}<br>	∧ -2.reserved not in {=}<br>	∧ -3.reserved = /<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ +2.reserved not in {>}<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles not in {DECLARATION, IF, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 485.` |
| 70 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if, import, {}<br>	∧ -2.label not in {<-space>}<br>	∧ -2.reserved = module<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +2.reserved not in {>}<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles not in {DECLARATION, IF, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.976. Support: 104.` |
| 71 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, import, {}<br>	∧ -2.label not in {<-space>}<br>	∧ -2.reserved not in {=, module}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = <<br>	∧ +2.reserved not in {>}<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION, IF, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 896.` |
| 72 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, if, {}<br>	∧ -2.label not in {<-space>}<br>	∧ -2.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = <<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {DECLARATION, IF, INCOMPLETE, OPERATOR, VARIABLE}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.816. Support: 182.` |
| 73 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = )<br>	∧ -2.label not in {<-space>}<br>	∧ -2.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = .<br>	∧ +2.reserved not in {>}<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles not in {DECLARATION, IF, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.862. Support: 692.` |
| 74 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = )<br>	∧ -2.label not in {<-space>}<br>	∧ -2.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +2.reserved not in {>}<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles not in {DECLARATION, IF, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.960. Support: 4049.` |
| 75 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {), ,, :, ;, const, if, import, {}<br>	∧ -2.internal_type = NumericLiteral<br>	∧ -2.label not in {<-space>}<br>	∧ -2.reserved not in {=}<br>	∧ -2.roles in {LITERAL}<br>	∧ -3.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +2.reserved not in {>}<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles not in {DECLARATION, IF, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 108.` |
| 76 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = [<br>	∧ -2.label not in {<-space>}<br>	∧ -2.reserved not in {=}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -3.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +2.reserved not in {>}<br>	∧ +2.length ≤ 1<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles not in {DECLARATION, IF, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.956. Support: 375.` |
| 77 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {), ,, :, ;, const, if, import, {}<br>	∧ -2.label not in {<-space>}<br>	∧ -2.reserved not in {=}<br>	∧ -2.roles not in {LITERAL}<br>	∧ -3.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +2.reserved not in {>}<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles not in {DECLARATION, IF, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 42449.` |
| 78 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -2.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length = 0<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {DECLARATION, IF, OPERATOR, VARIABLE}<br>⇒ y = ⏎<br>Confidence: 0.955. Support: 99.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 9.756410256410257, "max_conf": 0.9996851682662964, "max_support": 42449, "min_conf": 0.8056994676589966, "min_support": 94, "num_rules": 78}}
```
</details>
