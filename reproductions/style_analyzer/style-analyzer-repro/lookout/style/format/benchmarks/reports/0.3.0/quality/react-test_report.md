# Test report for javascript / file:///tmp/top-repos-quality-repos-qazqevzg/react HEAD 1034e26fe5e42ba07492a736da7bdf5bf2108bc6

### Classification report

PPCR: 0.946

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.980| 0.990| 0.972| 0.985| 0.976| 46694| 47515| 0.983 |
| `␣` | 0.960| 0.976| 0.902| 0.968| 0.930| 15146| 16375| 0.925 |
| `'` | 0.968| 0.994| 0.973| 0.981| 0.971| 5877| 6006| 0.979 |
| `⏎` | 0.839| 0.927| 0.750| 0.881| 0.792| 4445| 5489| 0.810 |
| `⏎␣⁻␣⁻` | 0.952| 0.911| 0.880| 0.931| 0.914| 2699| 2793| 0.966 |
| `⏎␣⁺␣⁺` | 0.960| 0.756| 0.664| 0.846| 0.785| 2603| 2966| 0.878 |
| `⏎⏎` | 0.868| 0.400| 0.217| 0.547| 0.348| 1018| 1872| 0.544 |
| `"` | 1.000| 0.909| 0.909| 0.952| 0.952| 460| 460| 1.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 61| 76| 0.803 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 9| 0.889 |
| `weighted avg` | 0.963| 0.964| 0.912| 0.962| 0.931| 79011| 83561| 0.946 |
| `macro avg` | 0.753| 0.686| 0.627| 0.709| 0.667| 79011| 83561| 0.946 |
| `micro avg` | 0.964| 0.964| 0.912| 0.964| 0.937| 79011| 83561| 0.946 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|821 |46205 |313 |15 |17 |54 |89 |1 |0 |0 |0 |
|1229 |182 |14778 |2 |170 |9 |4 |1 |0 |0 |0 |
|129 |18 |0 |5843 |0 |16 |0 |0 |0 |0 |0 |
|1044 |104 |150 |10 |4119 |2 |0 |60 |0 |0 |0 |
|363 |443 |92 |90 |9 |1969 |0 |0 |0 |0 |0 |
|94 |148 |48 |0 |45 |0 |2458 |0 |0 |0 |0 |
|854 |29 |4 |32 |546 |0 |0 |407 |0 |0 |0 |
|0 |0 |0 |42 |0 |0 |0 |0 |418 |0 |0 |
|15 |25 |1 |0 |3 |0 |32 |0 |0 |0 |0 |
|1 |1 |6 |0 |0 |1 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9521640091116174, "precision": 1.0, "recall": 0.908695652173913, "support": 460}, "\u0027": {"f1-score": 0.9811098984132314, "precision": 0.9683460391117004, "recall": 0.9942147354092223, "support": 5877}, "macro avg": {"f1-score": 0.7090749666172457, "precision": 0.752680995424209, "recall": 0.686174636224418, "support": 79011}, "micro avg": {"f1-score": 0.9643847059270229, "precision": 0.9643847059270229, "recall": 0.9643847059270229, "support": 79011}, "weighted avg": {"f1-score": 0.9622385614225829, "precision": 0.96349220659385, "recall": 0.9643847059270229, "support": 79011}, "\u2205": {"f1-score": 0.9846668584641285, "precision": 0.9798536740536529, "recall": 0.9895275624277209, "support": 46694}, "\u23ce": {"f1-score": 0.8806927517639513, "precision": 0.8390710939091465, "recall": 0.9266591676040495, "support": 4445}, "\u23ce\u23ce": {"f1-score": 0.5474108944182918, "precision": 0.8678038379530917, "recall": 0.39980353634577603, "support": 1018}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8461538461538463, "precision": 0.9600195026816187, "recall": 0.7564348828275067, "support": 2603}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9307080651268458, "precision": 0.9516066589237321, "recall": 0.9107076695072249, "support": 2699}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 61}, "\u2423": {"f1-score": 0.9678433427205448, "precision": 0.9601091476091476, "recall": 0.9757031559487653, "support": 15146}},
  "cl_report_full": {"\"": {"f1-score": 0.9521640091116174, "precision": 1.0, "recall": 0.908695652173913, "support": 460}, "\u0027": {"f1-score": 0.9705980066445182, "precision": 0.9683460391117004, "recall": 0.9728604728604728, "support": 6006}, "macro avg": {"f1-score": 0.6668637241315897, "precision": 0.752680995424209, "recall": 0.6268197939745315, "support": 83561}, "micro avg": {"f1-score": 0.937393893167335, "precision": 0.9643847059270229, "recall": 0.911872763609818, "support": 83561}, "weighted avg": {"f1-score": 0.9306399219590356, "precision": 0.960865310229738, "recall": 0.911872763609818, "support": 83561}, "\u2205": {"f1-score": 0.9761276011408049, "precision": 0.9798536740536529, "recall": 0.9724297590234663, "support": 47515}, "\u23ce": {"f1-score": 0.7922677437968839, "precision": 0.8390710939091465, "recall": 0.750409910730552, "support": 5489}, "\u23ce\u23ce": {"f1-score": 0.3477146518581803, "precision": 0.8678038379530917, "recall": 0.21741452991452992, "support": 1872}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7849312338050627, "precision": 0.9600195026816187, "recall": 0.6638570465273095, "support": 2966}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9144345238095238, "precision": 0.9516066589237321, "recall": 0.8800572860723237, "support": 2793}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 76}, "\u2423": {"f1-score": 0.9303994711493058, "precision": 0.9601091476091476, "recall": 0.902473282442748, "support": 16375}},
  "ppcr": 0.945548760785534
}
```
</details>